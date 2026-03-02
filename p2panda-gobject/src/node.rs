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

// TODO: Should this just be PublicKey?
#[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
#[boxed_type(name = "P2pandaNodeId", nullable)]
pub struct NodeId(pub(crate) node::NodeId);

impl NodeId {
    pub fn from_data(data: [u8; 32]) -> Result<Self, glib::Error> {
        Ok(Self(node::NodeId::try_from(data).map_err(|error| {
            glib::Error::new(Error::Signature, &error.to_string())
        })?))
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
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Node {
        const NAME: &'static str = "P2pandaNode";
        type Type = super::Node;
    }

    impl ObjectImpl for Node {
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
                    glib::ParamSpecBoolean::builder("default-migrations")
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

                match pspec.name() {
                    "private-key" => value
                        .get::<Option<PrivateKey>>()
                        .expect("type conformity checked by `Object::set_property`")
                        .map(|private_key| builder.private_key(private_key.0)),
                    "database-url" => value
                        .get::<Option<&str>>()
                        .expect("type conformity checked by `Object::set_property`")
                        .map(|database_url| builder.database_url(database_url)),
                    "default-migrations" => {
                        let default_migrations = value
                            .get::<bool>()
                            .expect("type conformity checked by `Object::set_property`");

                        Some(builder.default_migrations(default_migrations))
                    }
                    /*"ack-policy" => {
                        let ack_policy = value
                            .get::<AckPolicy>()
                            .expect("type conformity checked by `Object::set_property`");
                        Some(builder.ack_policy(ack_policy.into()))
                    }*/
                    "network-id" => value
                        .get::<Option<NetworkId>>()
                        .expect("type conformity checked by `Object::set_property`")
                        .map(|network_id| builder.network_id(network_id.0)),
                    "relay-url" => value
                        .get::<Option<glib::Uri>>()
                        .expect("type conformity checked by `Object::set_property`")
                        .map(|relay_url| {
                            builder.relay_url(
                                node::RelayUrl::from_str(relay_url.to_str().as_str())
                                    .expect("Malformed URL"),
                            )
                        }),
                    "bootstrap" => value
                        .get::<Option<NodeId>>()
                        .expect("type conformity checked by `Object::set_property`")
                        .map(|bootstrap| builder.bootstrap(bootstrap.0)),
                    "mdns-mode" => {
                        let mdns_mode = value
                            .get::<MdnsDiscoveryMode>()
                            .expect("type conformity checked by `Object::set_property`");
                        Some(builder.mdns_mode(mdns_mode.into()))
                    }
                    _ => unimplemented!(),
                }
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
        default_migrations: bool,
        //ack_policy: AckPolicy,
        network_id: Option<&NetworkId>,
        relay_url: Option<&glib::Uri>,
        bootstrap: Option<&NodeId>,
        mdns_mode: MdnsDiscoveryMode,
    ) -> Self {
        glib::Object::builder()
            .property("private-key", private_key)
            .property("database-url", database_url)
            .property("default-migrations", default_migrations)
            //   .property("ack-policy", ack_policy)
            .property("network-id", network_id)
            .property("relay-url", relay_url)
            .property("bootstrap", bootstrap)
            .property("mdns-mode", mdns_mode)
            .build()
    }

    pub fn id(&self) -> NodeId {
        NodeId(self.node().id())
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
                    tokio::spawn(async move {
                        tokio::pin!(event_stream);
                        while let Some(_event) = event_stream.next().await {
                            if let Some(obj) = weak_obj.upgrade() {
                                // TODO: add the event to the signal
                                obj.emit_by_name::<()>("system-event", &[]);
                            }
                        }
                    });

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
