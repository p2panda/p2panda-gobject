pub mod error;
pub mod identity;
pub mod node;
pub mod topic;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::PublicKey;
    use crate::node::MdnsDiscoveryMode;
    use crate::node::NetworkId;
    use crate::node::Node;
    use crate::topic::Topic;
    use crate::topic::TopicFlags;
    use crate::topic::TopicId;
    use gio::prelude::CancellableExt;
    use gio::prelude::CancellableExtManual;
    use glib::Bytes;
    use glib::closure;
    use glib::object::ObjectExt;

    #[glib::async_test]
    async fn messages_between_nodes() {
        let cancel = gio::Cancellable::new();
        let network_id = NetworkId::new();
        let topic_id = TopicId::new();
        let node1 = Node::new(
            None,
            None,
            true,
            Some(&network_id),
            None,
            None,
            MdnsDiscoveryMode::Active,
        );

        node1.spawn().await.unwrap();

        let topic1 = Topic::new(&node1, &topic_id, TopicFlags::PERSISTENT);
        let cancel_clone = cancel.clone();
        topic1.connect_closure(
            "message",
            true,
            closure!(move |obj: Topic,
                           public_key: PublicKey,
                           datetime: glib::DateTime,
                           bytes: glib::Bytes| {
                println!("Got message {public_key:?} {datetime:?} {bytes:?}");
                cancel_clone.cancel();
                true
            }),
        );

        topic1.spawn().await.unwrap();

        let node2 = Node::new(
            None,
            None,
            true,
            Some(&network_id),
            None,
            None,
            MdnsDiscoveryMode::Active,
        );

        node2.spawn().await.unwrap();

        let topic2 = Topic::new(&node2, &topic_id, TopicFlags::PERSISTENT);
        topic2.spawn().await.unwrap();

        let v = vec![1, 2, 3];
        let b = Bytes::from(&v);
        topic2.publish(b).await.unwrap();
        cancel.future().await;
    }
}
