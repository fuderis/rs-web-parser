use crate::prelude::*;
use super::{ SearchEngine, Page };
use chromedriver_api::{ Session, Tab };

/// The google search engine
pub struct GoogleSearch {
    session: Arc<TokioMutex<Option<Session>>>,
    tab: Arc<TokioMutex<Tab>>,
}

impl SearchEngine for GoogleSearch {
    /// Creates and runs a search engine
    /// * `path`: file path to chromedriver
    async fn run(path: &str) -> Result<Self> {
        let free_port = std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()?.port().to_string();

        let mut session = Session::run(
            &free_port,
            Some(path),
            Some(&path!("$/WebSearch/Profile").to_str().unwrap()),
            false
        ).await?;
        
        let tab = session.open("https://google.com/").await?;
        
        Ok(Self {
            session: Arc::new(TokioMutex::new(Some(session))),
            tab,
        })
    }
    
    /// Search results by query
    async fn search(&mut self, query: &str) -> Result<Vec<Page>> {
        let mut tab = self.tab.lock().await;
        tab.open("https://google.com/").await?;
        
        let status = tab.inject::<bool>(&(str!() + r#"
            let input = document.querySelector('textarea');
            if (!input) { return false; }

            input.focus();
            input.value = ""# + query + r#"";

            input.dispatchEvent(new Event('input', { bubbles: true }));
            input.dispatchEvent(new Event('change', { bubbles: true }));

            let press = new KeyboardEvent('keydown', {
                bubbles: true,
                cancelable: true,
                key: 'Enter',
                code: 'Enter',
                charCode: 13,
                keyCode: 13
            });

            input.dispatchEvent(press);

            return true;
        "#)).await?;

        if !status {
            tab.close().await.ok();
            return Err(Error::SessionBroken.into());
        }

        sleep2(Duration::from_millis(1000)).await;

        // get search results:
        let results = tab.inject::<Vec<String>>(r##"
                let domains = new Set();
                let links = [];

                document.querySelectorAll("#main *[data-rpos] a[href]").forEach(elem => {
                    let href = elem.getAttribute("href");

                    if (href && href.startsWith("https://") && !["youtube.com","rutube.ru","vkvideo.ru"].some(site => href.includes(site))) {
                        try {
                            let domain = new URL(href).hostname;
                            
                            if (!domains.has(domain)) {
                                domains.add(domain);
                                links.push(href);
                            }
                        } catch (e) {
                        }
                    }
                });

                return links;
            "##).await;
        
        // unlock tab:
        drop(tab);

        // convert URL's:
        let pages = results?.into_iter()
            .map(|url| Page::new(self.tab.clone(), url))
            .collect::<Vec<_>>();

        Ok(pages)
    }

    /// Closes a search engine session
    async fn stop(self) -> Result<()> {
        if let Some(session) = self.session.lock().await.take() {
            session.close().await?;
        }

        Ok(())
    }
}
