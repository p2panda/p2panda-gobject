/* Taken from glycin https://gitlab.gnome.org/GNOME/glycin/-/blob/main/libglycin/src/common.rs */
use gio::ffi::GAsyncResult;
use gio::prelude::*;
use glib::{ffi::gpointer, gobject_ffi::GObject};

type GAsyncReadyCallback = unsafe extern "C" fn(*mut GObject, *mut GAsyncResult, gpointer);

struct GPointerSend(pub gpointer);

unsafe impl Send for GPointerSend {}

pub struct GAsyncReadyCallbackSend {
    callback: GAsyncReadyCallback,
    user_data: GPointerSend,
}

unsafe impl Send for GAsyncReadyCallbackSend {}

impl GAsyncReadyCallbackSend {
    pub fn new(callback: GAsyncReadyCallback, user_data: gpointer) -> Self {
        Self {
            callback,
            user_data: GPointerSend(user_data),
        }
    }

    pub unsafe fn call<'a, P, O>(&self, obj: &'a O, res: *mut gio::ffi::GAsyncResult)
    where
        O: glib::translate::ToGlibPtr<'a, *mut P> + IsA<glib::Object>,
    {
        unsafe {
            let obj_ptr = obj.as_ptr();
            (self.callback)(obj_ptr as *mut _, res, self.user_data.0)
        }
    }
}
