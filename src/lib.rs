#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod error;   pub use error::{ Result, Error };
pub mod prelude;

pub mod parser;  pub use parser::{ User, Document, Node, Nodes };
