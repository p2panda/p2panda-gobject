use std::cell::{Cell, OnceCell};
use std::sync::OnceLock;

use glib::Properties;
use glib::clone;
use glib::prelude::*;
use glib::subclass::prelude::*;

use p2panda::{node, streams};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

use crate::{
    error::Error,
    identity::PublicKey,
    node::{Node, NodeId},
};

#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaTopicId", nullable)]
#[derive(Default)]
pub struct TopicId(pub(crate) node::Topic);

impl TopicId {
    pub fn new() -> Self {
        TopicId(node::Topic::new())
    }

    pub fn from_data(data: [u8; 32]) -> Self {
        TopicId(node::Topic::from(data))
    }
}

// TODO: Should the Hash actually be expose? It's not really needed.
#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaHash", nullable)]
pub struct Hash(pub(crate) node::Hash);

type Body = Vec<u8>;

#[glib::flags(name = "P2pandaTopicFlags")]
pub enum TopicFlags {
    NONE = 0,
    PERSISTENT = (1 << 0),
    EPHEMERAL = (1 << 1),
    FROM_START = (1 << 2),
}

impl Default for TopicFlags {
    fn default() -> Self {
        Self::NONE
    }
}

pub mod imp {
    use super::*;

    use glib::subclass::Signal;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::Topic)]
    pub struct Topic {
        pub(crate) spawned: Mutex<bool>,
        #[property(get, set, construct_only)]
        pub(super) node: OnceCell<Node>,
        #[property(get, set, construct_only)]
        pub(super) topic_id: OnceCell<TopicId>,
        #[property(get, set, construct_only)]
        flags: Cell<TopicFlags>,
        pub(super) stream_publisher: OnceCell<streams::StreamPublisher<Body>>,
        pub(super) ephemeral_publisher: OnceCell<streams::EphemeralStreamPublisher<Body>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Topic {
        const NAME: &'static str = "P2pandaTopic";
        type Type = super::Topic;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Topic {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder("message")
                        .param_types([
                            //Hash::static_type(),
                            PublicKey::static_type(),
                            glib::DateTime::static_type(),
                            glib::Bytes::static_type(),
                        ])
                        .return_type::<bool>()
                        .build(),
                    Signal::builder("ephemeral-message")
                        .param_types([
                            PublicKey::static_type(),
                            glib::DateTime::static_type(),
                            glib::Bytes::static_type(),
                        ])
                        .build(),
                    Signal::builder("sync-started")
                        .param_types([
                            NodeId::static_type(),
                            glib::types::Type::U64,
                            glib::types::Type::U64,
                            glib::types::Type::U64,
                            glib::types::Type::U64,
                            glib::types::Type::U64,
                        ])
                        .build(),
                    Signal::builder("sync-ended")
                        .param_types([NodeId::static_type(), glib::types::Type::U64])
                        .build(),
                    Signal::builder("error")
                        .param_types([glib::Error::static_type()])
                        .build(),
                ]
            })
        }
    }
}

unsafe impl Send for Topic {}
unsafe impl Sync for Topic {}

glib::wrapper! {
    pub struct Topic(ObjectSubclass<imp::Topic>);
}

impl Topic {
    pub fn new(node: &Node, topic_id: &TopicId, flags: TopicFlags) -> Self {
        glib::Object::builder()
            .property("node", node)
            .property("topic-id", topic_id)
            .property("flags", flags)
            .build()
    }

    pub async fn publish(&self, bytes: glib::Bytes) -> Result<Hash, glib::Error> {
        self.node()
            .runtime()
            .spawn(clone!(
                #[strong(rename_to = obj)]
                self,
                async move {
                    // TODO: should we automatically spawn the topic, or just fail?
                    //self.spawn().await?;
                    if !*obj.imp().spawned.lock().await {
                        return Err(glib::Error::new(
                            Error::NotSpawned,
                            "This topic was not spawned.",
                        ));
                    }

                    let Some(publisher) = obj.imp().stream_publisher.get() else {
                        return Err(glib::Error::new(
                            Error::HasNoEphemeral,
                            "This topic was created without the persistent flag set.",
                        ));
                    };
                    publisher
                        .publish(bytes.to_vec())
                        .await
                        .map(|res| Hash(res.hash()))
                        .map_err(|error| glib::Error::new(Error::Publish, &error.to_string()))
                }
            ))
            .await
            .unwrap()
    }

    pub async fn publish_ephemeral(&self, bytes: glib::Bytes) -> Result<(), glib::Error> {
        self.node()
            .runtime()
            .spawn(clone!(
                #[strong(rename_to = obj)]
                self,
                async move {
                    // TODO: should we automatically spawn the topic, or just fail?
                    //self.spawn().await?;
                    if !*obj.imp().spawned.lock().await {
                        return Err(glib::Error::new(
                            Error::NotSpawned,
                            "This topic was not spawned.",
                        ));
                    }

                    let Some(ephemeral_stream) = obj.imp().ephemeral_publisher.get() else {
                        return Err(glib::Error::new(
                            Error::HasNoEphemeral,
                            "This topic was created without the ephemeral flag set.",
                        ));
                    };
                    ephemeral_stream
                        .publish(bytes.to_vec())
                        .await
                        .map_err(|error| glib::Error::new(Error::Publish, &error.to_string()))
                }
            ))
            .await
            .unwrap()
    }

    pub async fn spawn(&self) -> Result<(), glib::Error> {
        self.node()
            .runtime()
            .spawn(clone!(
                #[strong(rename_to = obj)]
                self,
                async move {
                    let mut spawned = obj.imp().spawned.lock().await;

                    if *spawned {
                        return Ok(());
                    }

                    // TODO: should we automatically spawn the node, or just fail? It's not that expensive
                    // probably.
                    //let node = self.node();
                    //node.spawn().await?;

                    let node_obj = obj.node();
                    let node = node_obj.node();
                    let topic_id = obj.topic_id().0;
                    let flags = obj.flags();

                    if flags.contains(TopicFlags::PERSISTENT) {
                        let offset = if flags.contains(TopicFlags::FROM_START) {
                            streams::Offset::Start
                        } else {
                            streams::Offset::Frontier
                        };
                        let (publisher, mut subscription) =
                            node.stream_from(topic_id, offset).await.map_err(|error| {
                                glib::Error::new(Error::SpawnTopic, &error.to_string())
                            })?;

                        obj.imp()
                            .stream_publisher
                            .set(publisher)
                            .expect("Topic can be spawned only once");

                        let obj_weak = obj.downgrade();
                        // TODO: we need to abort this spawn when the Topic is dropped
                        tokio::spawn(async move {
                            while let Some(event) = subscription.next().await {
                                if let Some(obj) = obj_weak.upgrade() {
                                    obj.emit_signal_for_event(event).await;
                                } else {
                                    break;
                                }
                            }
                        });
                    }

                    if flags.contains(TopicFlags::EPHEMERAL) {
                        let (publisher, mut subscription) =
                            node.ephemeral_stream(topic_id).await.map_err(|error| {
                                glib::Error::new(Error::SpawnTopic, &error.to_string())
                            })?;

                        obj.imp()
                            .ephemeral_publisher
                            .set(publisher)
                            .expect("Topic can be spawned only once");

                        let obj_weak = obj.downgrade();
                        // TODO: we need to abort this spawn when the Topic is dropped
                        tokio::spawn(async move {
                            while let Some(message) = subscription.next().await {
                                if let Some(obj) = obj_weak.upgrade() {
                                    obj.emit_signal_for_ephemeral_message(message);
                                } else {
                                    break;
                                }
                            }
                        });
                    }

                    *spawned = true;

                    Ok(())
                }
            ))
            .await
            .unwrap()
    }

    async fn emit_signal_for_event(&self, event: streams::StreamEvent<Body>) {
        match event {
            // TODO: Expose source to the user
            streams::StreamEvent::Processed {
                operation,
                source: _,
            } => {
                let _hash = operation.id();
                let datetime =
                    glib::DateTime::from_unix_utc_usec(operation.timestamp() as i64).unwrap();
                let bytes: glib::Bytes = operation.message().into();
                //TODO: invoke on the thread owning the main context
                let ack = self.emit_by_name::<bool>(
                    "message",
                    &[
                        //&Hash(hash),
                        &PublicKey(operation.author()),
                        &datetime,
                        &bytes,
                    ],
                );
                if ack {
                    // TODO: not implemented yet
                    //operation.ack().await;
                }
            }
            // TODO: Expose topic_sessions to the user
            streams::StreamEvent::SyncStarted {
                remote_node_id,
                session_id,
                incoming_operations,
                outgoing_operations,
                incoming_bytes,
                outgoing_bytes,
                topic_sessions: _,
            } => {
                self.emit_by_name::<()>(
                    "sync-started",
                    &[
                        &NodeId::from(remote_node_id),
                        &session_id,
                        &incoming_operations,
                        &outgoing_operations,
                        &incoming_bytes,
                        &outgoing_bytes,
                    ],
                );
            }
            // TODO: Expose ignored fileds
            streams::StreamEvent::SyncEnded {
                remote_node_id,
                session_id,
                sent_operations: _,
                received_operations: _,
                sent_bytes: _,
                received_bytes: _,
                sent_bytes_topic_total: _,
                received_bytes_topic_total: _,
                error: _,
            } => {
                self.emit_by_name::<()>(
                    "sync-ended",
                    &[&NodeId::from(remote_node_id), &session_id],
                );
            }
            streams::StreamEvent::DecodingFailed { error, .. } => {
                // TODO: figure out whether we need to expose more about the event
                self.emit_by_name::<()>(
                    "error",
                    &[&glib::Error::new(Error::Decoding, &error.to_string())],
                );
            }
            streams::StreamEvent::ReplayFailed { error, .. } => {
                self.emit_by_name::<()>("error", &[&glib::Error::new(Error::Replay, &error)]);
            }
        }
    }

    fn emit_signal_for_ephemeral_message(&self, message: streams::EphemeralMessage<Body>) {
        let bytes: glib::Bytes = message.body().into();
        let datetime = glib::DateTime::from_unix_utc_usec(message.timestamp() as i64).unwrap();
        self.emit_by_name::<()>(
            "ephemeral-message",
            &[&PublicKey(message.author()), &datetime, &bytes],
        );
    }
}
