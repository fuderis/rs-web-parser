use crate::prelude::*;
use chromedriver_api::Session;

/// A website page reader
#[derive(Debug, Clone)]
pub struct Page {
    session: Arc<TokioMutex<Session>>,
    pub url: String,
}

impl Page {
    /// Creates a new website page instance
    pub fn new<S: Into<String>>(session: Arc<TokioMutex<Session>>, url: S) -> Self {
        Self {
            session,
            url: url.into(),
        }
    }
    
    /// Open and read website page
    pub fn read(&self) -> Result<Document> {
        todo!()
    }
}
