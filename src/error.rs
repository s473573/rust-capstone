use std::fmt;

#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    Image(String),
    Misc(String),
}

// controlling error output 
impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliError::Io(err) => write!(f, "IO Error: {}", err),
            CliError::Image(msg) => write!(f, "Image Error: {}", msg),
            CliError::Misc(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err)
    }
}
impl From<image::ImageError> for CliError {
    fn from(err: image::ImageError) -> Self {
        CliError::Image(err.to_string())
    }
}