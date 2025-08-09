use crate::prelude::*;
use super::Cites;
use chromedriver_api::{ Session, Tab };

/// A search engine options
pub trait SearchParams {
    /// Creates a new instance
    fn new() -> Self;
    
    /// Search engine URL
    fn url(&self) -> String;

    /// Start search script
    fn search(&self, query: &str) -> String;

    /// Parse results script
    fn parse(&self) -> String;
}

/// The search engine
pub struct SearchEngine<P>
where
    P: SearchParams
{
    params: P,
    session: Arc<TokioMutex<Option<Session>>>,
    tab: Arc<TokioMutex<Tab>>,
}

impl<P> SearchEngine<P>
where
    P: SearchParams
{
    /// Creates a new search engine session
    /// * path: path to the chromedriver (None = to use system PATH)
    /// * profile: path to save Chrome profile cookies (None = to not save cookies)
    /// * headless: run chromedriver without interface
    pub async fn new(path: Option<&str>, profile: Option<&str>, headless: bool) -> Result<Self> {
        let params = P::new();
        let free_port = std::net::TcpListener::bind("127.0.0.1:0")?.local_addr()?.port().to_string();

        let mut session = Session::run(
            &free_port,
            path,
            profile,
            headless
        ).await?;
        
        let tab = session.open(params.url()).await?;
        
        Ok(Self {
            params,
            session: Arc::new(TokioMutex::new(Some(session))),
            tab,
        })
    }
    
    /// Search results by query
    /// * query: a user search query
    /// * black_list: ignore these sites
    /// * sleep: waiting time for realism
    pub async fn search(&mut self, query: &str, black_list: &[&str], sleep: u64) -> Result<Cites> {
        let mut tab = self.tab.lock().await;
        tab.open(self.params.url()).await?;
        
        let query = query.trim().replace("\"", "'");
        let status = tab.inject::<bool>(&self.params.search(&query)).await?;

        if !status {
            tab.close().await.ok();
            return Err(Error::FailedGetResults.into());
        }

        sleep2(Duration::from_millis(100 + sleep)).await;

        // get search results:
        let results = tab.inject::<Vec<String>>(&self.params.parse()).await?;
        
        // unlock tab:
        drop(tab);

        Ok(Cites::new(self.tab.clone(), results, black_list))
    }

    /// Closes a search engine session
    pub async fn stop(self) -> Result<()> {
        if let Some(session) = self.session.lock().await.take() {
            session.close().await?;
        }

        Ok(())
    }
}
