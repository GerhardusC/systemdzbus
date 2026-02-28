use zbus::Connection;

use crate::{
    ManagerProxy,
    errors::SystemdError,
    systemctl::{connection_level::ConnectionLevel, unit::Unit},
};

pub struct SystemCtl<'a> {
    manager_proxy: Option<ManagerProxy<'a>>,
    connection_level: ConnectionLevel,
}

impl<'a> SystemCtl<'a> {
    pub fn new(connection_level: ConnectionLevel) -> Self {
        SystemCtl {
            manager_proxy: None,
            connection_level: connection_level,
        }
    }

    pub async fn init(&mut self) -> Result<(), SystemdError> {
        let connection = self.connection_level.get_connection().await?;
        let proxy = ManagerProxy::new(&connection).await?;
        self.manager_proxy = Some(proxy);
        Ok(())
    }

    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub async fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        let Some(proxy) = &self.manager_proxy else {
            return Err(SystemdError::InitialisationError);
        };
        let units = proxy.list_units().await?;
        Ok(units.into_iter().map(Into::into).collect())
    }
}

impl ConnectionLevel {
    async fn get_connection(&self) -> Result<Connection, SystemdError> {
        let connection = match self {
            ConnectionLevel::UserLevel => Connection::session().await?,
            ConnectionLevel::SystemLevel => Connection::system().await?,
        };

        Ok(connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_list_units() {
        smol::block_on(async {
            let mut system_ctl = SystemCtl::new(ConnectionLevel::UserLevel);

            system_ctl
                .init()
                .await
                .expect("Should be able to init connection");

            let units = system_ctl.list_units().await;

            assert!(units.is_ok());

            let units = units.expect("Units are OK by now");

            assert!(units.len() > 0);
        });
    }
}
