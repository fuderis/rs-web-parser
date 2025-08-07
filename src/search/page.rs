use crate::prelude::*;
use chromedriver_api::Tab;

/// A website page reader
#[derive(Debug, Clone)]
pub struct Page {
    tab: Arc<TokioMutex<Tab>>,
    pub url: String,
}

impl Page {
    /// Creates a new website page instance
    pub fn new<S: Into<String>>(tab: Arc<TokioMutex<Tab>>, url: S) -> Self {
        Self {
            tab,
            url: url.into(),
        }
    }

    /// Fast read website page
    pub async fn read(&self) -> Result<Document> {
        Document::read(&self.url, User::random()).await
    }
    
    /// Open and read website page
    pub async fn open_and_read(&self) -> Result<Document> {
        // open URL:
        let mut tab = self.tab.lock().await;
        tab.open(&self.url).await?;

        // get HTML:
        let html = tab.inject::<String>(r#"
            return document.querySelector("html").outerHTML;
        "#).await?;

        // unlock tab:
        drop(tab);
        
        Document::parse(&html)
    }
}
