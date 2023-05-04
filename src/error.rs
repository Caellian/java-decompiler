use thiserror::Error;

#[derive(Error, Debug)]
pub enum ManifestParseError {
    #[error("misplaced continuation line")]
    MisplacedContinuation,
    #[error("invalid header field")]
    InvalidHeader,
    #[error("invalid manifest entry")]
    InvalidEntry,

    #[error(transparent)]
    IOError {
        #[from]
        inner: std::io::Error,
    },
}
