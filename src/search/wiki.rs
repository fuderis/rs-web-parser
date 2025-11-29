use crate::prelude::*;
use super::{ SearchOptions };

/// The 'Wikipedia' search engine
pub struct Wiki;

impl SearchOptions for Wiki {
    /// Creates a new instance
    fn new() -> Self {
        Self {}
    }
    
    /// Search engine URL
    fn url(&self) -> String {
        str!("https://wikipedia.org/w/index.php?search=")
    }

    /// Start search script
    fn search(&self, query: &str) -> String {
        str!() + r##"
            try {
                let form = document.querySelector('body form#search');
                let input = form.querySelector('input[name="search"]');

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

                document.querySelectorAll('.mw-search-results a[href]').forEach(elem => {
                    let href = elem.getAttribute("href");

                    if (href && !href.startsWith("https://")) {
                        links.push('https://wikipedia.org' + href);
                    }
                });

                return links;
            } catch {
                return [];
            }
        "##
    }
}
