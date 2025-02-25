use std::{error::Error as StdError, num::TryFromIntError};
use thiserror::Error;

#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
use crate::storage::StorageError;

/// Convenience type alias for usage within apalis.
///
pub(crate) type BoxDynError = Box<dyn StdError + 'static + Send + Sync>;

/// Represents an error that is returned from an job.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum JobError {
    /// An error occurred during execution.
    #[error("Job Failed: {0}")]
    Failed(#[source] BoxDynError),

    #[cfg(feature = "storage")]
    #[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
    /// An error communicating with storage.
    #[error("Error communicating with storage: {0}")]
    Storage(#[from] StorageError),

    /// A generic IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A parse error occurred during execution.
    #[error("TryFromIntError {0}")]
    IntParseError(#[from] TryFromIntError),

    /// A job is missing some context and yet it was requested during execution.
    #[cfg(feature = "extensions")]
    #[error("MissingContext: {0}")]
    MissingContext(String),
}

/// Represents a [JobStream] error.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum JobStreamError {
    /// An error occurred during streaming.
    #[error("Broken Pipe: {0}")]
    BrokenPipe(#[source] BoxDynError),
}
