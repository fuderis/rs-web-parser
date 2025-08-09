use crate::prelude::*;
use super::{ SearchEngine, Cites };
use chromedriver_api::{ Session, Tab };

const SEARCH_URL: &str = "https://www.bing.com/";

/// The bing search engine
pub struct BingSearch {
    session: Arc<TokioMutex<Option<Session>>>,
    tab: Arc<TokioMutex<Tab>>,
}

impl SearchEngine for BingSearch {
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
        
        let tab = session.open(SEARCH_URL).await?;
        
        Ok(Self {
            session: Arc::new(TokioMutex::new(Some(session))),
            tab,
        })
    }
    
    /// Search results by query
    /// * query: a user search query
    /// * black_list: ignore these sites
    /// * sleep: waiting time for realism
    async fn search(&mut self, query: &str, black_list: &[&str], sleep: u64) -> Result<Cites> {
        let mut tab = self.tab.lock().await;
        tab.open(SEARCH_URL).await?;
        
        let status = tab.inject::<bool>(&(str!() + r#"
            let form = document.querySelector('form[action="/search"]');
            let input = form.querySelector('input[type="search"]');
            if (!form || !input) { return false; }

            input.focus();
            input.value = ""# + query + r#"";

            input.dispatchEvent(new Event('input', { bubbles: true }));
            input.dispatchEvent(new Event('change', { bubbles: true }));

            form.submit();

            return true;
        "#)).await?;

        if !status {
            tab.close().await.ok();
            return Err(Error::SessionBroken.into());
        }

        sleep2(Duration::from_millis(100 + sleep)).await;

        // get search results:
        let results = tab.inject::<Vec<String>>(&(str!() + r##"
                let links = [];

                document.querySelectorAll("main a[href] cite").forEach(elem => {
                    let href = elem.textContent
                        .replaceAll("&nbsp;", " ")
                        .replaceAll(/\s+›\s+/g, "/")
                        .trim();

                    if (href && href.startsWith("https://")
                    && !"## + &json::to_string(black_list).unwrap()+ r##".some(site => href.includes(site))) {
                        links.push(href);
                    }
                });

                return links;
            "##)).await?;
        
        // unlock tab:
        drop(tab);

        Ok(Cites::new(self.tab.clone(), results))
    }

    /// Closes a search engine session
    async fn stop(self) -> Result<()> {
        if let Some(session) = self.session.lock().await.take() {
            session.close().await?;
        }

        Ok(())
    }
}
