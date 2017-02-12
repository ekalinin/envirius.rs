
#[macro_use]
extern crate clap;

mod common;
pub mod cmd;

pub use common::Nv;

pub fn new(root: String) -> Nv {
    common::Nv::new(root)
}