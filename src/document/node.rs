use crate::prelude::*;
use super::Nodes;

/// The HTML node
#[derive(Debug, Clone)]
pub struct Node<'a> {
    element: scraper::ElementRef<'a>,
    selector: scraper::Selector
}

impl<'a> Node<'a> {
    /// Creates a new Node from scraper::ElementRef
    pub(crate) fn new(element: scraper::ElementRef<'a>) -> Self {
        Self {
            element,
            selector: scraper::Selector::parse("*").unwrap()
        }
    }

    /// Select HTML node by CSS selector
    pub fn select(&self, selector: &'static str) -> Result<Option<Node<'a>>> {
        let sel = scraper::Selector::parse(selector).map_err(Error::from)?;
        
        let node = self.element
            .select(&sel)
            .next()
            .map(Node::new);
        
        Ok(node)
    }

    /// Select HTML nodes by CSS selector
    pub fn select_all(&mut self, selector: &'static str) -> Result<Option<Nodes>> {
        self.selector = scraper::Selector::parse(selector).map_err(Error::from)?;
        let mut nodes = self.element.select(&self.selector).peekable();

        if nodes.peek().is_some() {
            Ok(Some(Nodes::new(None, Some(nodes))))
        } else {
            Ok(None)
        }
    }

    /// Returns a node parent
    pub fn parent(&self) -> Option<Node<'a>> {
        self.element.parent()
            .and_then(|node| scraper::ElementRef::wrap(node))
            .map(Node::new)
    }

    /// Returns a node attribute
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.element.value().attr(name)
    }

    /// Returns a node text contents
    pub fn text(&self) -> String {
        self.element.text().collect()
    }

    /// Returns a node inner HTML
    pub fn html(&self) -> String {
        self.element.html()
    }
}
