use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid Url (expected format {expected:?}, got {found:?})")]
    InvalidUrl { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}
