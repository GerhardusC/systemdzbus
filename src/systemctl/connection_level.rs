use zbus::{Connection, blocking::Connection as ConnectionBlocking};

use crate::errors::SystemdError;
pub enum ConnectionLevel {
    /// Create a Connection to the session/user message bus.
    UserLevel,
    /// Create a Connection to the system-wide message bus.
    SystemLevel,
}

impl ConnectionLevel {
    pub async fn get_connection(&self) -> Result<Connection, SystemdError> {
        let connection = match self {
            ConnectionLevel::UserLevel => Connection::session().await?,
            ConnectionLevel::SystemLevel => Connection::system().await?,
        };

        Ok(connection)
    }

    pub fn get_connection_blocking(&self) -> Result<ConnectionBlocking, SystemdError> {
        let connection = match self {
            ConnectionLevel::UserLevel => ConnectionBlocking::session()?,
            ConnectionLevel::SystemLevel => ConnectionBlocking::system()?,
        };

        Ok(connection)
    }
}
