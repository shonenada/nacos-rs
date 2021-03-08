#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate derive_builder;

mod client;
pub mod structs;

pub use client::NacosService;
