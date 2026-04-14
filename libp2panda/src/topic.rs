use std::ffi::c_uint;

use futures::future::{AbortHandle, Abortable};
use gio::ffi::{GAsyncReadyCallback, GAsyncResult, GTask};
use gio::prelude::CancellableExtManual;
use glib::ffi::{GError, GType, gpointer};
use glib::object::Cast;
use glib::object::ObjectType;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::types::StaticType;
use p2panda_gobject::{node, topic};

use crate::node::P2pandaNode;
use crate::utils::*;

pub type P2pandaTopic = <topic::imp::Topic as ObjectSubclass>::Instance;

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_topic_get_type() -> GType {
    <topic::Topic as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_topic_flags_get_type() -> GType {
    <topic::TopicFlags as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_new(
    node: *mut P2pandaNode,
    topic_id: *mut topic::TopicId,
    flags: c_uint,
) -> *mut P2pandaTopic {
    unsafe {
        let node = node::Node::from_glib_none(node);
        let topic_id = Option::<topic::TopicId>::from_glib_none(topic_id).unwrap_or_default();
        let flags = topic::TopicFlags::from_glib(flags);

        topic::Topic::new(&node, &topic_id, flags).into_glib_ptr()
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_get_node(topic: *mut P2pandaTopic) -> *mut P2pandaNode {
    unsafe {
        let topic = topic::Topic::from_glib_borrow(topic);
        topic.node().into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_get_topic_id(
    topic: *mut P2pandaTopic,
) -> *const topic::TopicId {
    unsafe {
        let topic = topic::Topic::from_glib_borrow(topic);
        topic.topic_id().into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_get_flags(topic: *mut P2pandaTopic) -> u32 {
    unsafe {
        let topic = topic::Topic::from_glib_borrow(topic);
        topic.flags().into_glib()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_spawn_async(
    topic: *mut P2pandaTopic,
    cancellable: *mut gio::ffi::GCancellable,
    callback: GAsyncReadyCallback,
    user_data: gpointer,
) {
    unsafe {
        let obj = topic::Topic::from_glib_none(topic);
        let cancellable: Option<gio::Cancellable> = from_glib_none(cancellable);
        let callback = callback.map(|callback| GAsyncReadyCallbackSend::new(callback, user_data));

        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let cancel_signal = if let Some(cancellable) = &cancellable {
            cancellable.connect_cancelled(move |_| abort_handle.abort())
        } else {
            None
        };

        let cancellable_ = cancellable.clone();
        let closure = move |task: gio::Task<bool>, obj: Option<&topic::Topic>| {
            if let (Some(cancel_signal), Some(cancellable)) = (cancel_signal, cancellable) {
                cancellable.disconnect_cancelled(cancel_signal);
            }

            if let Some(callback) = callback {
                let result = task.upcast_ref::<gio::AsyncResult>().as_ptr();
                callback.call(obj.unwrap(), result);
            }
        };

        let task = gio::Task::new(Some(&obj), cancellable_.as_ref(), closure);

        glib::MainContext::ref_thread_default().spawn(async move {
            let _ = Abortable::new(
                async {
                    let res = obj.spawn().await.map(|_| true);
                    task.return_result(res);
                },
                abort_registration,
            )
            .await;
        });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_spawn_finish(
    _topic: *mut P2pandaTopic,
    res: *mut GAsyncResult,
    error: *mut *mut GError,
) -> glib::ffi::gboolean {
    unsafe {
        let task = gio::Task::<bool>::from_glib_none(res as *mut GTask);

        match task.propagate() {
            Ok(_) => true as i32,
            Err(e) => {
                if !error.is_null() {
                    *error = e.into_glib_ptr();
                }
                false as i32
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_publish_async(
    topic: *mut P2pandaTopic,
    bytes: *mut glib::ffi::GBytes,
    ephemeral: glib::ffi::gboolean,
    cancellable: *mut gio::ffi::GCancellable,
    callback: GAsyncReadyCallback,
    user_data: gpointer,
) {
    unsafe {
        let obj = topic::Topic::from_glib_none(topic);
        let bytes = glib::Bytes::from_glib_none(bytes);
        let cancellable: Option<gio::Cancellable> = from_glib_none(cancellable);
        let callback = callback.map(|callback| GAsyncReadyCallbackSend::new(callback, user_data));

        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let cancel_signal = if let Some(cancellable) = &cancellable {
            cancellable.connect_cancelled(move |_| abort_handle.abort())
        } else {
            None
        };

        let cancellable_ = cancellable.clone();
        let closure = move |task: gio::Task<bool>, obj: Option<&topic::Topic>| {
            if let (Some(cancel_signal), Some(cancellable)) = (cancel_signal, cancellable) {
                cancellable.disconnect_cancelled(cancel_signal);
            }

            if let Some(callback) = callback {
                let result = task.upcast_ref::<gio::AsyncResult>().as_ptr();
                callback.call(obj.unwrap(), result);
            }
        };

        let task = gio::Task::new(Some(&obj), cancellable_.as_ref(), closure);

        glib::MainContext::ref_thread_default().spawn(async move {
            let _ = Abortable::new(
                async {
                    let res = if ephemeral != 0 {
                        obj.publish_ephemeral(bytes).await.map(|_| true)
                    } else {
                        obj.publish(bytes).await.map(|_| true)
                    };
                    task.return_result(res);
                },
                abort_registration,
            )
            .await;
        });
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_publish_finish(
    _topic: *mut P2pandaTopic,
    res: *mut GAsyncResult,
    error: *mut *mut GError,
) -> glib::ffi::gboolean {
    unsafe {
        let task = gio::Task::<bool>::from_glib_none(res as *mut GTask);

        match task.propagate() {
            Ok(_) => true as i32,
            Err(e) => {
                if !error.is_null() {
                    *error = e.into_glib_ptr();
                }
                false as i32
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_topic_id_get_type() -> GType {
    <topic::TopicId as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_id_new() -> *mut topic::TopicId {
    topic::TopicId::new().into_glib_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_id_new_from_data(
    data: *const [u8; 32],
) -> *mut topic::TopicId {
    unsafe { topic::TopicId::from_data(*data).into_glib_ptr() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_id_get_data(topic_id: *mut topic::TopicId) -> *const [u8; 32] {
    unsafe {
        let topic_id = topic::TopicId::from_glib_none(topic_id);
        topic_id.into_glib_ptr() as *const [u8; 32]
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_id_copy(
    topic_id: *mut topic::TopicId,
) -> *mut topic::TopicId {
    unsafe {
        topic::TopicId::from_glib_none(topic_id)
            .clone()
            .into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_topic_id_free(topic_id: *mut topic::TopicId) {
    unsafe {
        drop(topic::TopicId::from_glib_full(topic_id));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_hash_get_type() -> GType {
    <topic::Hash as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_hash_copy(hash: *mut topic::Hash) -> *mut topic::Hash {
    unsafe { topic::Hash::from_glib_none(hash).clone().into_glib_ptr() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_hash_free(hash: *mut topic::Hash) {
    unsafe {
        drop(topic::Hash::from_glib_full(hash));
    }
}
