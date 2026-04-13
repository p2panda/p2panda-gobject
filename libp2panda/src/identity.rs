use glib::ffi::{GError, GType};
use glib::translate::*;
use glib::types::StaticType;
use p2panda_gobject::identity;

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_private_key_get_type() -> GType {
    <identity::PrivateKey as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_private_key_copy(
    private_key: *const identity::PrivateKey,
) -> *const identity::PrivateKey {
    unsafe {
        identity::PrivateKey::from_glib_none(private_key)
            .clone()
            .into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_private_key_free(private_key: *mut identity::PrivateKey) {
    unsafe {
        drop(identity::PrivateKey::from_glib_full(private_key));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_private_key_new() -> *const identity::PrivateKey {
    identity::PrivateKey::new().into_glib_ptr()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_private_key_get_public_key(
    private_key: *const identity::PrivateKey,
) -> *mut identity::PublicKey {
    unsafe {
        let private_key = identity::PrivateKey::from_glib_none(private_key);
        private_key.public_key().into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_private_key_new_from_data(
    data: *const [u8; 32],
) -> *const identity::PrivateKey {
    unsafe { identity::PrivateKey::from_data(*data).into_glib_ptr() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_private_key_get_data(
    private_key: *const identity::PrivateKey,
) -> *const [u8; 32] {
    unsafe {
        let private_key = identity::PrivateKey::from_glib_none(private_key);
        private_key.into_glib_ptr() as *const [u8; 32]
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_public_key_get_type() -> GType {
    <identity::PublicKey as StaticType>::static_type().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_public_key_copy(
    public_key: *const identity::PublicKey,
) -> *const identity::PublicKey {
    unsafe {
        identity::PublicKey::from_glib_none(public_key)
            .clone()
            .into_glib_ptr()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_public_key_free(public_key: *mut identity::PublicKey) {
    unsafe {
        drop(identity::PublicKey::from_glib_full(public_key));
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_public_key_new_from_data(
    data: *const [u8; 32],
    error: *mut *mut GError,
) -> *const identity::PublicKey {
    unsafe {
        match identity::PublicKey::from_data(*data) {
            Ok(public_key) => public_key.into_glib_ptr(),
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
pub unsafe extern "C" fn p2panda_public_key_get_data(
    public_key: *const identity::PublicKey,
) -> *const [u8; 32] {
    unsafe {
        let public_key = identity::PublicKey::from_glib_none(public_key);
        public_key.into_glib_ptr() as *const [u8; 32]
    }
}
