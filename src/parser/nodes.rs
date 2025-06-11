use super::Node;

use std::iter::{ Iterator, Peekable };

/// The HTML nodes iterator
#[derive(Debug, Clone)]
pub struct Nodes<'a, 'b> {
    iter_from_doc: Option<Peekable<scraper::html::Select<'a, 'b>>>,
    iter_from_node: Option<Peekable<scraper::element_ref::Select<'a, 'b>>>,
}

impl<'a, 'b> Nodes<'a, 'b> {
    pub(crate) fn new(iter_from_doc: Option<Peekable<scraper::html::Select<'a, 'b>>>, iter_from_node: Option<Peekable<scraper::element_ref::Select<'a, 'b>>>) -> Self {
        Self {
            iter_from_doc,
            iter_from_node
        }
    }
}

impl<'a, 'b> Iterator for Nodes<'a, 'b> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(iter) = self.iter_from_doc.as_mut() {
            if let Some(item) = iter.next() {
                return Some(Node::new(item));
            }
        }

        else if let Some(iter) = self.iter_from_node.as_mut() {
            if let Some(item) = iter.next() {
                return Some(Node::new(item));
            }
        }

        None
    }
}
