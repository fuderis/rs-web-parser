use crate::{ prelude::*, };
use super::Page;

/// The search engine alias
pub type DynSearchEngine = Box<dyn SearchEngine + Send + Sync>;

/// The search engine trait
pub trait SearchEngine: Sized {
    /// Creates a new search engine session
    /// * path: path to the chromedriver (None = to use system PATH)
    /// * profile: path to save Chrome profile cookies (None = to not save cookies)
    /// * headless: run chromedriver without interface
    fn new(path: Option<&str>, profile: Option<&str>, headless: bool) -> impl std::future::Future<Output = Result<Self>> + Send;
    
    /// Search results by query
    /// * query: a user search query
    /// * black_list: ignore these sites
    /// * sleep: waiting time for realism
    fn search(&mut self, query: &str, black_list: &[&str], sleep: u64) -> impl std::future::Future<Output = Result<Vec<Page>>> + Send;

    /// Stops search engine session
    fn stop(self) -> impl std::future::Future<Output = Result<()>> + Send;
}
