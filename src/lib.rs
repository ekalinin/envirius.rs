
#[macro_use]
extern crate clap;

pub mod common;
pub mod cmd;

pub use common::Nv;

// Shortcut for creating a new Envirius wrapper
pub fn new(root: String) -> Nv {
    common::Nv::new(root)
}