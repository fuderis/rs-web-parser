use crate::prelude::*;
use super::{ SearchOptions };

/// The 'Duck' search engine
pub struct Duck;

impl SearchOptions for Duck {
    /// Creates a new instance
    fn new() -> Self {
        Self {}
    }
    
    /// Search engine URL
    fn url(&self) -> String {
        str!("https://duckduckgo.com/")
    }

    /// Start search script
    fn search(&self, query: &str) -> String {
        str!() + r##"
            try {
                let form = document.querySelector('main form#searchbox_homepage');
                let input = form.querySelector('input[aria-autocomplete]');

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

                document.querySelectorAll('body a[href] p').forEach(elem => {
                    let href = elem.textContent
                        .replaceAll("&nbsp;", " ")
                        .replaceAll(/\s+›\s+/g, "/")
                        .trim();

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
