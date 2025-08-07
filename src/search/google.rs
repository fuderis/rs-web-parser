use crate::prelude::*;
use super::{ SearchEngine, Page };
use chromedriver_api::Session;

/// The google search engine
pub struct GoogleSearch {
    session: Arc<TokioMutex<Session>>,
}

impl SearchEngine for GoogleSearch {
    /// Creates and runs a search engine
    async fn run() -> Result<Self> {
        let free_port = std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()?.port().to_string();

        let session = Session::run(
            &free_port,
            Some("/bin/chromedriver/chromedriver.exe"),
            Some("C:/Users/Admin/AppData/Local/Google/Chrome/Profiles/Profile1"),
            true
        ).await?;
        
        Ok(Self {
            session: Arc::new(TokioMutex::new(session)),
        })
    }
    
    /// Search results by query
    async fn search(&self, query: &str) -> Result<Vec<Page>> {
        todo!()
    }
}
