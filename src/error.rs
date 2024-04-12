#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Crashed during an io manipulation: {0}.")]
    Io(#[from] std::io::Error),
    #[error("Error encountered while modifying an image: {0}")]
    Image(#[from] image::error::ImageError),
    #[error("An error occured! {0}")]
    Misc(String),
}
