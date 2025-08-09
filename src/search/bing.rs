use crate::prelude::*;
use super::{ SearchParams };

/// The 'Bing' search engine
pub struct Bing;

impl SearchParams for Bing {
    /// Creates a new instance
    fn new() -> Self {
        Self {}
    }
    
    /// Search engine URL
    fn url(&self) -> String {
        str!("https://www.bing.com/")
    }

    /// Start search script
    fn search(&self, query: &str) -> String {
        str!() + r##"
            try {
                let form = document.querySelector('form[action="/search"]');
                let input = form.querySelector('input[type="search"]');

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

                document.querySelectorAll('main a[href] cite').forEach(elem => {
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
