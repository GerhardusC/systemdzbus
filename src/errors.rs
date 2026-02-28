use thiserror::Error;

#[derive(Debug, Error)]
pub enum SystemdError {
    #[error("Error occurred inside of the Zbus library.")]
    DbusError(#[from] zbus::Error),

    #[error("Connection not initialised")]
    InitialisationError,
}
