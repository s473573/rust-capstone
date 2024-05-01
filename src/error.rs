#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Crashed during an io manipulation: {0}.")]
    Io(#[from] std::io::Error),
    #[error("Error encountered while modifying an image: {0}")]
    Image(#[from] image::error::ImageError),
    #[error("Something wrong with the crypto!")]
    Crypto,
    #[error("An error occured! {0}")]
    Misc(String),
}

// something about this error type structure is preventing me from using the macro
impl From<ring::error::Unspecified> for CliError {
    fn from(_: ring::error::Unspecified) -> Self { CliError::Crypto }
}