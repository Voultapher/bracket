//! Errors generated when rendering templates.
use crate::error::{HelperError, IoError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Unable to resolve partial name from '{0}'")]
    PartialNameResolve(String),
    #[error("Partial '{0}' not found")]
    PartialNotFound(String),
    #[error(transparent)]
    Helper(#[from] HelperError),
    #[error(transparent)]
    Io(#[from] IoError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl From<std::io::Error> for RenderError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(IoError::Io(err))
    }
}

impl PartialEq for RenderError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::PartialNotFound(ref s), Self::PartialNotFound(ref o)) => {
                s == o
            }
            _ => false,
        }
    }
}

impl Eq for RenderError {}