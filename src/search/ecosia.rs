use crate::prelude::*;
use super::{ SearchParams };

/// The 'Ecosia' search engine
pub struct Ecosia;

impl SearchParams for Ecosia {
    /// Creates a new instance
    fn new() -> Self {
        Self {}
    }
    
    /// Search engine URL
    fn url(&self) -> String {
        str!("https://www.ecosia.org/")
    }

    /// Start search script
    fn search(&self, query: &str) -> String {
        str!() + r##"
            try {
                let form = document.querySelector('form[action="/search"]');
                let input = form.querySelector('input[data-test-id="search-form-input"]');

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
            let links = [];

            document.querySelectorAll('main a[href][data-test-id="result-link"]').forEach(elem => {
                let href = elem.getAttribute("href");

                if (href && href.startsWith("https://")) {
                    links.push(href);
                }
            });

            return links;
        "##
    }
}
