#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;

macro_rules! skip_assert_initialized {
    () => {};
}

/// No-op.
macro_rules! assert_initialized_main_thread {
    () => {};
}

mod auto;
pub use auto::*;

pub mod builders {
    pub use super::auto::builders::*;
}
