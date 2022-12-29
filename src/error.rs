//! Error crate for Application

#[derive(thisError::Error, Debug)]
pub enum Error {
    /// For starter only! Remove as code matures.
    #[error("Generic error: {0}")]
    Generic(String),
    /// For starter only! Remove as code matures.
    #[error("Static error: {0}")]
    Static(&'static str),

    // Input/Output error
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
