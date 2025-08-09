#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod error;      pub use error::{ Result, Error };
pub mod prelude;

pub mod document;   pub use document::{ User, Document, Node, Nodes };

#[cfg(feature = "search")]
pub mod search;
