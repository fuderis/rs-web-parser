use macron::{ Display, Error, From };

/// The std result
pub type StdResult<T, E> = std::result::Result<T, E>;
/// The result alias
pub type Result<T> = StdResult<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// The application error
#[derive(Debug, Display, Error, From)]
#[from(scraper::error::SelectorErrorKind<'static>, "Self::Scraper(value.to_string())")]
pub enum Error {
    #[from]
    String(String),

    Scraper(String),
}
