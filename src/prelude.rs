#![allow(unused_imports)]

pub use crate::{ Result, Error, User, Document };
#[cfg(feature = "search")]
pub use crate::{ SearchEngine, GoogleSearch, };

pub(crate) use std::format as fmt;
pub(crate) use std::collections::HashMap;
pub(crate) use std::path::{ Path, PathBuf };
pub(crate) use std::sync::{ Arc, Mutex };
pub(crate) use tokio::sync::{ Mutex as TokioMutex };
pub(crate) use tokio::time::{ sleep as sleep2, Duration };

pub(crate) use macron::*;
pub(crate) use serde::{ Serialize, Deserialize, de::DeserializeOwned };
pub(crate) use serde_json as json;
pub(crate) use json::{ json, Value };
pub(crate) use once_cell::sync::{ Lazy, OnceCell };
