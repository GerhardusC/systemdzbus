use zbus::Connection;

use crate::{
    ManagerProxy,
    errors::SystemdError,
    systemctl::{connection_level::ConnectionLevel, unit::Unit, unit_file::UnitFile},
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

    /// Get access to the manager proxy directly. This allows you to call all the functions defined
    /// and documented in the ManagerProxy type, which is basically everything in
    /// org.freedesktop.systemd1.Manager. Here you will only get strings and numbers with no
    /// meaning attached to them, hopefully in future this will not be a required method.
    pub async fn get_manager_proxy(&self) -> Result<&ManagerProxy<'a>, SystemdError> {
        self.manager_proxy
            .as_ref()
            .ok_or(SystemdError::InitialisationError)
    }

    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub async fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        let Some(proxy) = &self.manager_proxy else {
            return Err(SystemdError::InitialisationError);
        };
        let units = proxy.list_units().await?;
        Ok(units.into_iter().map(Into::into).collect())
    }

    /// Returns an array of unit names and their enablement status. Note that ListUnit() returns a list of units currently loaded into memory, while ListUnitFiles() returns a list of unit
    /// files that were found on disk. Note that while most units are read directly from a unit file with the same name, some units are not backed by files and some files (templates) cannot directly be loaded
    /// as units but need to be instantiated instead.
    pub async fn list_unit_files(&self) -> Result<Vec<UnitFile>, SystemdError> {
        let Some(proxy) = &self.manager_proxy else {
            return Err(SystemdError::InitialisationError);
        };

        let unit_files = proxy.list_unit_files().await?;
        Ok(unit_files.into_iter().map(Into::into).collect())
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
    fn can_list_unit_files() {
        smol::block_on(async {
            let mut system_ctl = SystemCtl::new(ConnectionLevel::UserLevel);

            system_ctl
                .init()
                .await
                .expect("Should be able to init connection");

            let unit_files = system_ctl.list_unit_files().await;

            assert!(unit_files.is_ok());

            let unit_files = unit_files.expect("Unit files should exist at this point");

            assert!(!unit_files.is_empty())
        })
    }

    #[test]
    fn can_use_manager_proxy_directly() {
        smol::block_on(async {
            let mut system_ctl = SystemCtl::new(ConnectionLevel::UserLevel);

            system_ctl
                .init()
                .await
                .expect("Should be able to init connection");

            let proxy = system_ctl
                .get_manager_proxy()
                .await
                .expect("Initialised at this point");

            let state = proxy.get_unit_file_state("dbus.service").await;

            assert!(state.is_ok());
        });
    }

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

            assert!(!units.is_empty());
        });
    }
}
