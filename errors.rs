use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    // ... other possible errors ...
}
