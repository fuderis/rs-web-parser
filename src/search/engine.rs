use crate::{ prelude::*, };
use super::Page;

/// The search engine alias
pub type DynSearchEngine = Box<dyn SearchEngine + Send + Sync>;

/// The search engine trait
pub trait SearchEngine: Sized {
    /// Creates a new search engine
    fn run(path: &str) -> impl std::future::Future<Output = Result<Self>> + Send;
    
    /// Search results by query
    fn search(&mut self, query: &str) -> impl std::future::Future<Output = Result<Vec<Page>>> + Send;

    /// Stops search engine session
    fn stop(self) -> impl std::future::Future<Output = Result<()>> + Send;
}
