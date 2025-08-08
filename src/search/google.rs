use crate::prelude::*;
use super::{ SearchEngine, Page };
use chromedriver_api::{ Session, Tab };

/// The google search engine
pub struct GoogleSearch {
    session: Arc<TokioMutex<Option<Session>>>,
    tab: Arc<TokioMutex<Tab>>,
}

impl SearchEngine for GoogleSearch {
    /// Creates a new search engine session
    /// * path: path to the chromedriver (None = to use system PATH)
    /// * profile: path to save Chrome profile cookies (None = to not save cookies)
    /// * headless: run chromedriver without interface
    async fn new(path: Option<&str>, profile: Option<&str>, headless: bool) -> Result<Self> {
        let free_port = std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()?.port().to_string();

        let mut session = Session::run(
            &free_port,
            path,
            profile,
            headless
        ).await?;
        
        let tab = session.open("https://google.com/").await?;
        
        Ok(Self {
            session: Arc::new(TokioMutex::new(Some(session))),
            tab,
        })
    }
    
    /// Search results by query
    /// * query: a user search query
    /// * black_list: ignore these sites
    /// * sleep: waiting time for realism
    async fn search(&mut self, query: &str, black_list: &[&str], sleep: u64) -> Result<Vec<Page>> {
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

        sleep2(Duration::from_millis(sleep)).await;

        // get search results:
        let results = tab.inject::<Vec<String>>(&(str!() + r##"
                let links = [];

                document.querySelectorAll("#main *[data-rpos] a[href]").forEach(elem => {
                    let href = elem.getAttribute("href");

                    if (href && href.startsWith("https://") && !"## + &json::to_string(black_list).unwrap()+ r##".some(site => href.includes(site))) {
                        links.push(href);
                    }
                });

                return links;
            "##)).await;
        
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
