#![feature(custom_derive)]

#![crate_name = "adapton_graph"]
#![crate_type = "lib"]

pub mod graph;

pub mod adapton_graph {
    pub use super::*;
}

