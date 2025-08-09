use crate::prelude::*;
use super::{ SearchParams };

/// The 'Yahoo' search engine
pub struct Yahoo;

impl SearchParams for Yahoo {
    /// Creates a new instance
    fn new() -> Self {
        Self {}
    }
    
    /// Search engine URL
    fn url(&self) -> String {
        str!("https://www.yahoo.com/")
    }

    /// Start search script
    fn search(&self, query: &str) -> String {
        str!() + r##"
            try {
                let form = document.querySelector('header form[role="search"]');
                let input = form.querySelector('input[autofocus]');

                form.removeAttribute('target');

                input.focus();
                input.value = ""## + query + r##"";

                input.dispatchEvent(new Event('input', { bubbles: true }));
                input.dispatchEvent(new Event('change', { bubbles: true }));

                form.submit();

                return true;
            } catch {
                return false;
            }
        "##
    }

    /// Parse results script
    fn parse(&self) -> String {
        str!() + r##"
            try {
                let links = [];

                document.querySelectorAll('#main #web a[href][referrerpolicy="origin"]').forEach(elem => {
                    let href = elem.getAttribute("href");

                    if (href && href.startsWith("https://")) {
                        links.push(href);
                    }
                });

                return links;
            } catch {
                return [];
            }
        "##
    }
}
