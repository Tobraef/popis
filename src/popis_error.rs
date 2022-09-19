use thiserror::Error;

pub type Result<T> = std::result::Result<T, PopisError>;

#[derive(Error, Debug)]
pub enum PopisError {
    #[error("The government site is not responding or it's API has changed: {0}")]
    WebRequest(#[from] reqwest::Error),
    #[error("HTML parsing error, the document structure on the website might have changed: {0}")]
    HtmlParsing(String),
    #[error("Couldn't connect to the database: {0}")]
    DbConnectionError(String),
}
