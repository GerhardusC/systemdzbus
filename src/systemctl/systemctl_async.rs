use zbus::Connection;

use crate::{
    ManagerProxy,
    errors::SystemdError,
    systemctl::{
        connection_level::ConnectionLevel,
        unit::Unit,
        unit_file::{EnablementStatus, UnitFile},
    },
};

pub struct SystemCtlBuilder {
    connection_level: ConnectionLevel,
}

impl SystemCtlBuilder {
    pub fn new() -> Self {
        Self {
            connection_level: ConnectionLevel::UserLevel,
        }
    }

    pub fn with_system_connection_level(mut self) -> Self {
        self.connection_level = ConnectionLevel::SystemLevel;
        self
    }

    pub async fn init<'a>(self) -> Result<SystemCtl<'a>, SystemdError> {
        let connection = self.connection_level.get_connection().await?;
        let proxy = ManagerProxy::new(&connection).await?;
        Ok(SystemCtl {
            manager_proxy: proxy,
            connection_level: self.connection_level,
        })
    }
}

impl Default for SystemCtlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SystemCtl<'a> {
    manager_proxy: ManagerProxy<'a>,
    #[allow(unused)]
    connection_level: ConnectionLevel,
}

impl<'a> SystemCtl<'a> {
    /// Get access to the manager proxy directly. This allows you to call all the functions defined
    /// and documented in the ManagerProxy type, which is basically everything in
    /// org.freedesktop.systemd1.Manager. Here you will only get strings and numbers with no
    /// meaning attached to them, hopefully in future this will not be a required method.
    pub fn get_manager_proxy(&self) -> &ManagerProxy<'a> {
        &self.manager_proxy
    }

    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub async fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        let proxy = self.get_manager_proxy();

        let units = proxy.list_units().await?;
        Ok(units.into_iter().map(Into::into).collect())
    }

    /// May be used to get the unit object path for a unit name. It takes the unit name and returns
    /// the object path. If a unit has not been loaded yet by this name this method will fail.
    pub async fn get_unit(&self, name: &str) -> Result<String, SystemdError> {
        let proxy = self.get_manager_proxy();
        let owned_object_path = proxy.get_unit(name).await?;

        Ok(owned_object_path.to_string())
    }

    /// Returns an array of unit names and their enablement status. Note that ListUnit() returns a list of units currently loaded into memory, while ListUnitFiles() returns a list of unit
    /// files that were found on disk. Note that while most units are read directly from a unit file with the same name, some units are not backed by files and some files (templates) cannot directly be loaded
    /// as units but need to be instantiated instead.
    pub async fn list_unit_files(&self) -> Result<Vec<UnitFile>, SystemdError> {
        let proxy = self.get_manager_proxy();

        let unit_files = proxy.list_unit_files().await?;
        Ok(unit_files.into_iter().map(Into::into).collect())
    }

    /// Returns the current enablement status of a specific unit file. The format of the string
    /// here is simply name.service, in other words, if you retrieved the unit files via
    /// list_unit_files, you may want to strip the prefix on the path to get the service name.
    pub async fn get_unit_file_state(&self, file: &str) -> Result<EnablementStatus, SystemdError> {
        let proxy = self.get_manager_proxy();
        let unit_file_state = proxy.get_unit_file_state(file).await?;

        Ok(unit_file_state.into())
    }

    /// May be invoked to reload all unit files.
    pub async fn reload(&self) -> Result<(), SystemdError> {
        let proxy = self.get_manager_proxy();
        Ok(proxy.reload().await?)
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
    fn can_get_unit() {
        smol::block_on(async {
            let system_ctl = SystemCtlBuilder::new()
                .init()
                .await
                .expect("Should be able to init connection");

            let unit = system_ctl
                .get_unit("dbus.service")
                .await
                .expect("Should be able to get dbus unit");

            assert!(unit.contains("dbus"));
        });
    }

    #[test]
    fn can_get_valid_unit_file_state() {
        smol::block_on(async {
            let system_ctl = SystemCtlBuilder::new()
                .init()
                .await
                .expect("Should be able to init connection");

            let units = system_ctl
                .list_unit_files()
                .await
                .expect("Should be able to list units");

            for unit in units {
                let file = unit
                    .path
                    .split('/')
                    .next_back()
                    .expect("Should not be empty string");
                let status = system_ctl
                    .get_unit_file_state(file)
                    .await
                    .expect("Should be able to get status");

                if let EnablementStatus::Other(_) = status {
                    panic!("All unit files should have a valid status returned");
                };
            }
        });
    }

    #[test]
    fn can_list_unit_files() {
        smol::block_on(async {
            let system_ctl = SystemCtlBuilder::new()
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
            let system_ctl = SystemCtlBuilder::new()
                .init()
                .await
                .expect("Should be able to init connection");

            let proxy = system_ctl.get_manager_proxy();

            let state = proxy.get_unit_file_state("dbus.service").await;

            assert!(state.is_ok());
        });
    }

    #[test]
    fn can_list_units() {
        smol::block_on(async {
            let system_ctl = SystemCtlBuilder::new()
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
