use crate::prelude::*;
use super::{ SearchOptions };

/// The 'Google' search engine
pub struct Google;

impl SearchOptions for Google {
    /// Creates a new instance
    fn new() -> Self {
        Self {}
    }
    
    /// Search engine URL
    fn url(&self) -> String {
        str!("https://www.google.com/")
    }

    /// Start search script
    fn search(&self, query: &str) -> String {
        str!() + r##"
            try {
                let input = document.querySelector('textarea');

                input.focus();
                input.value = ""## + query + r##"";

                input.dispatchEvent(new Event('input', { bubbles: true }));
                input.dispatchEvent(new Event('change', { bubbles: true }));

                input.dispatchEvent(new KeyboardEvent('keydown', {
                    bubbles: true,
                    cancelable: true,
                    key: 'Enter',
                    code: 'Enter',
                    charCode: 13,
                    keyCode: 13
                }));

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

                document.querySelectorAll('#main *[data-rpos] a[href]').forEach(elem => {
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
