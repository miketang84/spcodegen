#![feature(custom_derive, plugin)]

extern crate sporm;
extern crate serde;
extern crate serde_json;

mod writer;
mod meta;
pub mod generator;
pub mod table_json;
