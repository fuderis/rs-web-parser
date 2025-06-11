use macron::{ Display, Error, From };

// Result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// Parser error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display = "Error"]
    Error,
}
