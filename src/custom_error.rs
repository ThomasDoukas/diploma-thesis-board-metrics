// Rust module to manage all possible errors from back end.
// Will be using thiserror crate.

use iota_sdk::{
    client::Error as IotaClientError,
    types::block::Error as IotaBlockError
};
use std::{
    env::VarError, 
    string::FromUtf8Error
};
use serde_json::error::Error as SerdeError;

// Create the error type that represents all errors possible in our program.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Default error type.
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    // IO error created from std::io::Error.
    // Does not add any new information or messages.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    // Iota client error
    #[error(transparent)]
    IotaClientError(#[from] IotaClientError),

    // Iota block type error
    #[error(transparent)]
    IotaBlockError(#[from] IotaBlockError),

    // Environment Variable error
    #[error(transparent)]
    EnvError(#[from] VarError),

    // Converting u8 to String error
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),

    // Serde JSON Errro
    #[error(transparent)]
    SerdeError(#[from] SerdeError),
}