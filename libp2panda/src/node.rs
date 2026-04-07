use std::ffi::c_char;
use std::ffi::c_int;

use futures::future::{AbortHandle, Abortable};
use gio::ffi::{GAsyncReadyCallback, GAsyncResult, GTask};
use gio::prelude::CancellableExtManual;
use glib::ffi::{GError, GType, gpointer};
use glib::object::Cast;
use glib::object::ObjectType;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::types::StaticType;

use crate::utils::*;
use p2panda_gobject::{identity, node};

pub type P2pandaNode = <node::imp::Node as ObjectSubclass>::Instance;

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_node_get_type() -> GType {
    <node::Node as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_network_id_get_type() -> GType {
    <node::NetworkId as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_network_id_new() -> *mut node::NetworkId {
    node::NetworkId::new().into_glib_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_network_id_copy(
    network_id: *mut node::NetworkId,
) -> *mut node::NetworkId {
    unsafe {
        node::NetworkId::from_glib_none(network_id)
            .clone()
            .into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_network_id_free(network_id: *mut node::NetworkId) {
    unsafe {
        drop(node::NetworkId::from_glib_full(network_id));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_network_id_new_from_data(
    data: *const [u8; 32],
) -> *mut node::NetworkId {
    unsafe { node::NetworkId::from_data(*data).into_glib_ptr() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_network_id_get_data(
    network_id: *mut node::NetworkId,
) -> *const [u8; 32] {
    unsafe {
        let network_id = node::NetworkId::from_glib_none(network_id);
        network_id.into_glib_ptr() as *const [u8; 32]
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_node_id_get_type() -> GType {
    <node::NodeId as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_node_id_copy(node_id: *mut node::NodeId) -> *mut node::NodeId {
    unsafe {
        node::NodeId::from_glib_none(node_id)
            .clone()
            .into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_node_id_free(node_id: *mut node::NodeId) {
    unsafe {
        drop(node::NodeId::from_glib_full(node_id));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_node_id_new_from_data(
    data: *const [u8; 32],
    relay_url: *mut glib::ffi::GUri,
    error: *mut *mut GError,
) -> *mut node::NodeId {
    unsafe {
        let relay_url = Option::<glib::Uri>::from_glib_none(relay_url);

        match node::NodeId::from_data(*data, relay_url) {
            Ok(node_id) => node_id.into_glib_ptr(),
            Err(e) => {
                if !error.is_null() {
                    *error = e.into_glib_ptr();
                }
                std::ptr::null_mut()
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_node_id_get_data(node_id: *mut node::NodeId) -> *const [u8; 32] {
    unsafe {
        let node_id = node::NodeId::from_glib_none(node_id);
        node_id.into_glib_ptr() as *const [u8; 32]
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_node_new(
    private_key: *mut identity::PrivateKey,
    database_url: *const c_char,
    network_id: *mut node::NetworkId,
    relay_url: *mut glib::ffi::GUri,
    bootstrap_node: *mut node::NodeId,
    mdns_mode: c_int,
) -> *mut P2pandaNode {
    unsafe {
        let private_key = Option::<identity::PrivateKey>::from_glib_none(private_key);
        let database_url = if database_url.is_null() {
            None
        } else {
            glib::GStr::from_ptr_checked(database_url)
        };
        let network_id = Option::<node::NetworkId>::from_glib_none(network_id);
        let relay_url = Option::<glib::Uri>::from_glib_none(relay_url);
        let bootstrap_node = Option::<node::NodeId>::from_glib_none(bootstrap_node);
        let mdns_mode = node::MdnsDiscoveryMode::from_glib(mdns_mode);

        node::Node::new(
            private_key.as_ref(),
            database_url,
            network_id.as_ref(),
            relay_url.as_ref(),
            bootstrap_node.as_ref(),
            mdns_mode,
        )
        .into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_node_spawn_async(
    node: *mut P2pandaNode,
    cancellable: *mut gio::ffi::GCancellable,
    callback: GAsyncReadyCallback,
    user_data: gpointer,
) {
    unsafe {
        let obj = node::Node::from_glib_none(node);
        let cancellable: Option<gio::Cancellable> = from_glib_none(cancellable);
        let callback = GAsyncReadyCallbackSend::new(callback, user_data);

        let (abort_handle, abort_registration) = AbortHandle::new_pair();
        let cancel_signal = if let Some(cancellable) = &cancellable {
            cancellable.connect_cancelled(move |_| abort_handle.abort())
        } else {
            None
        };

        let cancellable_ = cancellable.clone();
        let closure = move |task: gio::Task<bool>, obj: Option<&node::Node>| {
            if let (Some(cancel_signal), Some(cancellable)) = (cancel_signal, cancellable) {
                cancellable.disconnect_cancelled(cancel_signal);
            }

            let result = task.upcast_ref::<gio::AsyncResult>().as_ptr();
            callback.call(obj.unwrap(), result);
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
pub unsafe extern "C" fn p2panda_node_spawn_finish(
    _node: *mut P2pandaNode,
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
pub extern "C" fn p2panda_mdns_discovery_mode_get_type() -> GType {
    <node::MdnsDiscoveryMode as StaticType>::static_type().into_glib()
}
