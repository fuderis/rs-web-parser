use crate::prelude::*;
use super::{ User, Node, Nodes };

/// The website HTML-document
#[derive(Debug, Clone)]
pub struct Document {
    html: scraper::Html,
    selector: scraper::Selector
}

impl Document {
    /// Parse HTML document from text
    pub fn parse(html: &str) -> Result<Self> {
        Ok(Self {
            html: scraper::Html::parse_document(&html),
            selector: scraper::Selector::parse("html").unwrap()
        })
    }
    
    /// Reads website page as HTML document
    pub async fn read(url: &str, user: User) -> Result<Self> {
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .header(reqwest::header::USER_AGENT, user.to_string())
            .send().await?
            .text().await?;

        Self::parse(&response)
    }

    /// Reads website page as simple text
    pub async fn text(url: &str, user: User) -> Result<String> {
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .header(reqwest::header::USER_AGENT, user.to_string())
            .send()
            .await?
            .text()
            .await?;

        Ok(response)
    }

    /// Reads website page as json
    pub async fn json<D: DeserializeOwned>(url: &str, user: User) -> Result<json::Result<D>> {
        let client = reqwest::Client::new();

        let response = client
            .get(url)
            .header(reqwest::header::USER_AGENT, user.to_string())
            .send()
            .await?
            .text()
            .await?;

        Ok(serde_json::from_str(&response))
    }

    /// Select HTML node by CSS selector
    pub fn select(&self, selector: &'static str) -> Result<Option<Node>> {
        let selector = scraper::Selector::parse(selector).map_err(Error::from)?;
        
        let node = self.html
            .select(&selector)
            .next()
            .map(Node::new);
        
        Ok(node)
    }

    /// Select HTML nodes by CSS selector
    pub fn select_all(&mut self, selector: &'static str) -> Result<Option<Nodes>> {
        self.selector = scraper::Selector::parse(selector).map_err(Error::from)?;
        let mut nodes = self.html.select(&self.selector).peekable();

        if nodes.peek().is_some() {
            Ok(Some(Nodes::new(Some(nodes), None)))
        } else {
            Ok(None)
        }
    }
}
