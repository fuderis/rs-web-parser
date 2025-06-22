use macron::{ Display, Error, From };

/// The std result
pub type StdResult<T, E> = std::result::Result<T, E>;
/// The result alias
pub type Result<T> = StdResult<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// The application error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    String(String),

    Scraper(String),
}

impl Error {
    pub fn from_scraper(e: scraper::error::SelectorErrorKind<'static>) -> Self {
        Self::Scraper(e.to_string())
    }
}
