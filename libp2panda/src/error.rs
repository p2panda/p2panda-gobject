use glib::error::ErrorDomain;
use glib::prelude::StaticType;
use glib::translate::IntoGlib;

use p2panda_gobject::error::Error;

#[unsafe(no_mangle)]
pub extern "C" fn p2panda_error_quark() -> glib::ffi::GQuark {
    Error::domain().into_glib()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn p2panda_error_get_type() -> glib::ffi::GType {
    Error::static_type().into_glib()
}
