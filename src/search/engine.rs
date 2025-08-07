use crate::{ prelude::*, };
use super::Page;

/// The web-search engine alias
pub type DynSearchEngine = Box<dyn SearchEngine + Send + Sync>;

/// The web-search engine trait
pub trait SearchEngine: Sized {
    /// Creates a new search engine
    async fn run() -> Result<Self>;
    
    /// Search results by query
    async fn search(&self, query: &str) -> Result<Vec<Page>>;
}
