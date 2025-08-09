use crate::prelude::*;
use chromedriver_api::Tab;
use futures::stream::{ FuturesUnordered, TryStreamExt };

/// A website content
#[derive(Debug, Clone)]
pub struct Content {
    pub url: String,
    pub text: String,
}

/// A searched websites list
#[derive(Debug, Clone)]
pub struct Cites {
    pub cites: Vec<Cite>,
}

impl Cites {
    /// Creates a new websites list by URL's
    pub(crate) fn new<S: Into<String>>(tab: Arc<TokioMutex<Tab>>, urls: Vec<S>) -> Self {
        Self {
            cites: urls.into_iter()
                .map(|url| Cite::new(tab.clone(), url.into()))
                .collect::<Vec<_>>()
        }
    }

    /// Reads a websites content
    pub async fn read(&self, count: usize, black_list: &[&str]) -> Result<Vec<Content>> {
        let cites_to_read = &self.cites[..self.cites.len().min(count)];

        let futures = cites_to_read.iter().map(|cite| async {
            // асинхронно читаем и парсим документ
            let doc = cite.read().await?;
            let main = doc.select("body")?.unwrap();

            let text = main.filter_text(black_list);

            Result::Ok(Content {
                url: cite.url.clone(),
                text,
            })
        });

        // Запускаем все задачи параллельно и собираем результаты
        let contents: Vec<Content> = futures
            .collect::<FuturesUnordered<_>>()
            .try_collect()
            .await?;

        Ok(contents)
    }

    /// Reads a websites content
    pub async fn read_all(&self, black_list: &[&str]) -> Result<Vec<Content>> {
        self.read(self.cites.len(), black_list).await
    }
}

/// A searched website wrap
#[derive(Debug, Clone)]
pub struct Cite {
    tab: Arc<TokioMutex<Tab>>,
    pub url: String,
}

impl Cite {
    /// Creates a new website page instance
    pub(crate) fn new<S: Into<String>>(tab: Arc<TokioMutex<Tab>>, url: S) -> Self {
        Self {
            tab,
            url: url.into(),
        }
    }

    /// Fast read website page
    pub async fn read(&self) -> Result<Document> {
        Document::read(&self.url, User::random()).await
    }
    
    /// Open and read website page
    pub async fn open_and_read(&self) -> Result<Document> {
        // open URL:
        let mut tab = self.tab.lock().await;
        tab.open(&self.url).await?;

        // get HTML:
        let html = tab.inject::<String>(r#"
            return document.querySelector("html").outerHTML;
        "#).await?;

        // unlock tab:
        drop(tab);
        
        Document::parse(&html)
    }
}
