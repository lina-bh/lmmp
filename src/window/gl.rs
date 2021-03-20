#![allow(unused_mut)]
#![allow(bare_trait_objects)]
#![allow(clippy::all)]
mod gl_raw {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub use gl_raw::*;
