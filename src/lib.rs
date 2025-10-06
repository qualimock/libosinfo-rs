#![cfg_attr(docsrs, feature(doc_cfg))]

pub use ffi;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

#[allow(unused_imports)]
mod auto;

pub use auto::*;

pub mod builders {
    pub use crate::auto::builders::*;
}
