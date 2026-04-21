use std::cell::{OnceCell, RefCell};
use std::str::FromStr;
use std::sync::OnceLock;

use glib::clone;
use glib::prelude::*;
use glib::subclass::prelude::*;

use p2panda::node;
use rand::prelude::*;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::task::AbortHandle;
use tokio_stream::StreamExt;

use crate::{error::Error, identity::PrivateKey};

#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaNetworkId", nullable)]
pub struct NetworkId(pub(crate) node::NetworkId);

impl Default for NetworkId {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkId {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self(rng.random())
    }

    pub fn from_data(data: [u8; 32]) -> Self {
        NetworkId(data as node::NetworkId)
    }
}

// FIXME: Would be nice to use `node::EndpointAddr`, but it is an iroh type
// or make `node::NodeId` contain the `relay_url`
// See: https://github.com/p2panda/p2panda/issues/1114
#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaNodeId", nullable)]
pub struct NodeId {
    id: node::NodeId,
    relay_url: Option<node::RelayUrl>,
}

impl From<node::PublicKey> for NodeId {
    fn from(public_key: node::PublicKey) -> Self {
        NodeId {
            id: public_key,
            relay_url: None,
        }
    }
}

impl NodeId {
    pub fn from_data(data: [u8; 32], relay_url: Option<glib::Uri>) -> Result<Self, glib::Error> {
        let id = node::NodeId::try_from(data)
            .map_err(|error| glib::Error::new(Error::Signature, &error.to_string()))?;
        let relay_url = relay_url.map(|relay_url| {
            node::RelayUrl::from_str(relay_url.to_str().as_str()).expect("Malformed URL")
        });

        Ok(Self { id, relay_url })
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "P2pandaMdnsDiscoveryMode")]
pub enum MdnsDiscoveryMode {
    #[default]
    Active,
    Passive,
}

impl From<MdnsDiscoveryMode> for node::MdnsDiscoveryMode {
    fn from(mode: MdnsDiscoveryMode) -> Self {
        match mode {
            MdnsDiscoveryMode::Active => Self::Active,
            MdnsDiscoveryMode::Passive => Self::Passive,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "P2pandaAckPolicy")]
pub enum AckPolicy {
    #[default]
    Explicit,
    Automatic,
}

impl From<AckPolicy> for node::AckPolicy {
    fn from(policy: AckPolicy) -> Self {
        match policy {
            AckPolicy::Explicit => Self::Explicit,
            AckPolicy::Automatic => Self::Automatic,
        }
    }
}

pub mod imp {
    use super::*;

    use glib::subclass::Signal;

    #[derive(Default)]
    pub struct Node {
        pub(super) spawned: Mutex<bool>,
        runtime: OnceLock<Runtime>,
        pub(super) node_builder: RefCell<Option<node::NodeBuilder>>,
        pub(super) node: OnceCell<node::Node>,
        pub(super) system_event_abort_handle: OnceCell<AbortHandle>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Node {
        const NAME: &'static str = "P2pandaNode";
        type Type = super::Node;
    }

    impl ObjectImpl for Node {
        fn dispose(&self) {
            if let Some(abort_handle) = self.system_event_abort_handle.get() {
                abort_handle.abort();
            }
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder("system-event").build()])
        }

        fn properties() -> &'static [glib::ParamSpec] {
            use std::sync::OnceLock;
            static PROPERTIES: OnceLock<Vec<glib::ParamSpec>> = OnceLock::new();
            PROPERTIES.get_or_init(|| {
                vec![
                    glib::ParamSpecBoxed::builder::<PrivateKey>("private-key")
                        .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                        .build(),
                    glib::ParamSpecString::builder("database-url")
                        .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                        .build(),
                    /*glib::ParamSpecEnum::builder::<AckPolicy>("ack-policy")
                    .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                    .build(),*/
                    glib::ParamSpecBoxed::builder::<NetworkId>("network-id")
                        .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                        .build(),
                    glib::ParamSpecBoxed::builder::<glib::Uri>("relay-url")
                        .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                        .build(),
                    glib::ParamSpecBoxed::builder::<NodeId>("bootstrap")
                        .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                        .build(),
                    glib::ParamSpecEnum::builder::<MdnsDiscoveryMode>("mdns-mode")
                        .flags(glib::ParamFlags::CONSTRUCT_ONLY | glib::ParamFlags::WRITABLE)
                        .build(),
                ]
            })
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.node_builder.replace_with(|builder| {
                let builder = builder.take().unwrap_or_else(node::Node::builder);

                let builder = match pspec.name() {
                    "private-key" => match value
                        .get::<Option<PrivateKey>>()
                        .expect("type conformity checked by `Object::set_property`")
                    {
                        Some(private_key) => builder.private_key(private_key.0),
                        None => builder,
                    },
                    "database-url" => match value
                        .get::<Option<&str>>()
                        .expect("type conformity checked by `Object::set_property`")
                    {
                        Some(database_url) => builder.database_url(database_url),
                        None => builder,
                    },
                    /*"ack-policy" => {
                        let ack_policy = value
                            .get::<AckPolicy>()
                            .expect("type conformity checked by `Object::set_property`");
                        builder.ack_policy(ack_policy.into())
                    }*/
                    "network-id" => match value
                        .get::<Option<NetworkId>>()
                        .expect("type conformity checked by `Object::set_property`")
                    {
                        Some(network_id) => builder.network_id(network_id.0),
                        None => builder,
                    },
                    "relay-url" => match value
                        .get::<Option<glib::Uri>>()
                        .expect("type conformity checked by `Object::set_property`")
                    {
                        Some(relay_url) => builder.relay_url(
                            node::RelayUrl::from_str(relay_url.to_str().as_str())
                                .expect("Malformed URL"),
                        ),
                        None => builder,
                    },
                    "bootstrap" => match value
                        .get::<Option<NodeId>>()
                        .expect("type conformity checked by `Object::set_property`")
                    {
                        Some(node_id) => {
                            let id = node_id.id;
                            let relay_url = node_id
                                .relay_url
                                .expect("A boostrap node needs a known relay url");

                            builder.bootstrap(id, relay_url)
                        }
                        None => builder,
                    },
                    "mdns-mode" => {
                        let mdns_mode = value
                            .get::<MdnsDiscoveryMode>()
                            .expect("type conformity checked by `Object::set_property`");
                        builder.mdns_mode(mdns_mode.into())
                    }
                    _ => unimplemented!(),
                };

                Some(builder)
            });
        }
    }

    impl Node {
        pub(super) fn runtime(&self) -> &Runtime {
            self.runtime.get_or_init(|| Runtime::new().unwrap())
        }
    }
}

glib::wrapper! {
    pub struct Node(ObjectSubclass<imp::Node>);
}

unsafe impl Send for Node {}
unsafe impl Sync for Node {}

impl Node {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        private_key: Option<&PrivateKey>,
        database_url: Option<&glib::GStr>,
        //ack_policy: AckPolicy,
        network_id: Option<&NetworkId>,
        relay_url: Option<&glib::Uri>,
        bootstrap: Option<&NodeId>,
        mdns_mode: MdnsDiscoveryMode,
    ) -> Self {
        glib::Object::builder()
            .property("private-key", private_key)
            .property("database-url", database_url)
            //   .property("ack-policy", ack_policy)
            .property("network-id", network_id)
            .property("relay-url", relay_url)
            .property("bootstrap", bootstrap)
            .property("mdns-mode", mdns_mode)
            .build()
    }

    pub fn id(&self) -> NodeId {
        NodeId::from(self.node().id())
    }

    pub async fn spawn(&self) -> Result<(), glib::Error> {
        self.runtime()
            .spawn(clone!(
                #[strong(rename_to = obj)]
                self,
                async move {
                    let mut spawned = obj.imp().spawned.lock().await;

                    if *spawned {
                        return Ok(());
                    }

                    let builder = obj.imp().node_builder.take().unwrap();

                    let node = builder
                        .spawn()
                        .await
                        .map_err(|error| glib::Error::new(Error::SpawnNode, &error.to_string()))?;
                    let event_stream = node
                        .event_stream()
                        .await
                        .map_err(|error| glib::Error::new(Error::SpawnNode, &error.to_string()))?;

                    obj.imp().node.set(node).unwrap();

                    let weak_obj = obj.downgrade();
                    // TODO: We need to abort this spawn when the Node is dropped (but the Runtime is
                    // dropped anyway)
                    let main_context = glib::MainContext::ref_thread_default();
                    let abort_handle = tokio::spawn(async move {
                        tokio::pin!(event_stream);
                        while let Some(_event) = event_stream.next().await {
                            let weak_obj = weak_obj.clone();
                            main_context.invoke(move || {
                                if let Some(obj) = weak_obj.upgrade() {
                                    // TODO: add the event to the signal
                                    obj.emit_by_name::<()>("system-event", &[]);
                                }
                            })
                        }
                    })
                    .abort_handle();

                    obj.imp()
                        .system_event_abort_handle
                        .set(abort_handle)
                        .expect("Node can be spawned only once");

                    *spawned = true;

                    Ok(())
                }
            ))
            .await
            .unwrap()
    }

    pub(crate) fn node(&self) -> &node::Node {
        self.imp().node.get().unwrap()
    }

    pub(crate) fn runtime(&self) -> &Runtime {
        self.imp().runtime()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_node() {
        let _node = Node::new(
            None,
            None,
            false,
            //AckPolicy::Explicit,
            None,
            None,
            None,
            MdnsDiscoveryMode::Passive,
        );
    }
}
