pub mod cites;      pub use cites::{ Cites, Cite, Content };

pub mod engine;     pub use engine::{ SearchEngine, DynSearchEngine };

pub mod google;     pub use google::GoogleSearch;
pub mod bing;       pub use bing::BingSearch;
pub mod duck;       pub use duck::DuckSearch;
