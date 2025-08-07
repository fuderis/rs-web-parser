use crate::prelude::*;
use super::Nodes;

static BLACK_LIST: Lazy<Vec<&'static str>> = Lazy::new(|| vec![
    "header", "footer", "style", "script", "noscript", "iframe", "button", "a", "img",
]);

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

    /// Returns node content from node excluding tags from black list
    pub fn filter_text(&self) -> String {
        Self::filter_elem_text(self.element)
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Returns node content from element excluding tags from black list
    fn filter_elem_text(node: scraper::element_ref::ElementRef) -> String {
        let tag_name = node.value().name();

        // filtering by black list:
        if BLACK_LIST.contains(&tag_name) {
            return String::new();
        }

        // collecting text:
        let mut result = String::new();

        for child in node.children() {
            match child.value() {
                scraper::node::Node::Text(text) => {
                    result.push_str(text);
                }
                scraper::node::Node::Element(_) => {
                    if let Some(child_element) = scraper::ElementRef::wrap(child) {
                        // Рекурсивно обходим элемент
                        result.push_str(&Self::filter_elem_text(child_element));
                    }
                }
                _ => {}
            }
        }
        result
    }
}
