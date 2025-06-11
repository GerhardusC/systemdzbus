use zbus::{Connection, blocking::Connection as ConnectionBlocking};
use crate::{errors::SystemdError, systemctl::Unit};
use crate::manager::{ManagerProxyBlocking, ManagerProxy};

pub enum ConnectionLevel {
    /// Create a Connection to the session/user message bus.
    UserLevel,
    /// Create a Connection to the system-wide message bus.
    SystemLevel,
}

impl ConnectionLevel {
    async fn get_connection(&self) -> Result<Connection, SystemdError> {
        let connection = match self {
            ConnectionLevel::UserLevel => Connection::session().await?,
            ConnectionLevel::SystemLevel => Connection::system().await?,
        };

        Ok(connection)
    }

    fn get_blocking_connection(&self) -> Result<ConnectionBlocking, SystemdError> {
        let connection = match self {
            ConnectionLevel::UserLevel => ConnectionBlocking::session()?,
            ConnectionLevel::SystemLevel => ConnectionBlocking::system()?,
        };

        Ok(connection)
    }
}

/// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
pub async fn list_units(level: ConnectionLevel) -> Result<Vec<Unit>, SystemdError> {
    let connection = level.get_connection().await?;
    let proxy = ManagerProxy::new(&connection).await?;
    let units = proxy.list_units().await?.into_iter().map(|item| {
        item.into()
    }).collect();
    Ok(units)
}

/// Synchronous version of `list_units()`
pub fn list_units_blocking(level: ConnectionLevel) -> Result<Vec<Unit>, SystemdError> {
    let connection = level.get_blocking_connection()?;
    let proxy = ManagerProxyBlocking::new(&connection)?;
    let units = proxy.list_units()?.into_iter().map(|item| {
        item.into()
    }).collect();
    Ok(units)
}

pub async fn list_sockets() -> Result<(), SystemdError> {
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_list_units_blocking () {
        let units = list_units_blocking(ConnectionLevel::UserLevel)
            .expect("Should be able to call list units");

        assert!(units.iter().count() > 0, "Should be able to list units with zbus connection");
    }

    #[test]
    fn should_list_units_async () {
        smol::block_on(async {
            let units = list_units(ConnectionLevel::UserLevel).await
                .expect("Should be able to call list units");

            assert!(units.iter().count() > 0, "Should be able to list units with zbus connection");
        })
    }
}

