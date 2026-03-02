use zbus::{Connection, zvariant::OwnedObjectPath};

use crate::{
    ManagerProxy,
    errors::SystemdError,
    systemctl::{
        connection_level::ConnectionLevel,
        unit::{Unit, UnitMode},
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

    /// Enqueues a start job and possibly depending jobs. It takes the unit to activate and a mode
    /// string as arguments. The mode needs to be one of "replace", "fail", "isolate", "ignore-dependencies", or
    /// "ignore-requirements". If "replace", the method will start the unit and its dependencies, possibly
    /// replacing already queued jobs that conflict with it. If "fail", the method will start the unit and its
    /// dependencies, but will fail if this would change an already queued job. If "isolate", the method will
    /// start the unit in question and terminate all units that aren't dependencies of it. If
    /// "ignore-dependencies", it will start a unit but ignore all its dependencies. If "ignore-requirements",
    /// it will start a unit but only ignore the requirement dependencies. It is not recommended to make use of
    /// the latter two options. On completion, this method returns the newly created job object.
    pub async fn start_unit(
        &self,
        name: &str,
        mode: UnitMode,
    ) -> Result<OwnedObjectPath, SystemdError> {
        Ok(self
            .get_manager_proxy()
            .start_unit(name, &mode.to_string())
            .await?)
    }

    /// Similar to StartUnit() but stops the specified unit rather than starting it. Note that the
    /// "isolate" mode is invalid for this method.
    pub async fn stop_unit(
        &self,
        name: &str,
        mode: UnitMode,
    ) -> Result<OwnedObjectPath, SystemdError> {
        if let UnitMode::Isolate = mode {
            return Err(SystemdError::IsolateModeUnavailable);
        };

        Ok(self
            .get_manager_proxy()
            .stop_unit(name, &mode.to_string())
            .await?)
    }

    /// RestartUnit method, takes in the mode, i.e. same as start unit, I quote:
    /// The mode needs to be one of "replace", "fail", "isolate", "ignore-dependencies", or
    /// "ignore-requirements". returns the object path of the restarted unit.
    pub async fn restart_unit(
        &self,
        name: &str,
        mode: UnitMode,
    ) -> Result<OwnedObjectPath, SystemdError> {
        Ok(self
            .get_manager_proxy()
            .restart_unit(name, &mode.to_string())
            .await?)
    }

    /// ReloadUnit(), RestartUnit(), TryRestartUnit(), ReloadOrRestartUnit(), or ReloadOrTryRestartUnit() may be used to restart and/or reload a unit. These methods take similar arguments as StartUnit(). Reloading is done only if the
    /// unit is already running and fails otherwise. If a service is restarted that isn't running, it will be started unless the "Try" flavor is used in which case a service that isn't running is not affected by the restart. The
    /// "ReloadOrRestart" flavors attempt a reload if the unit supports it and use a restart otherwise.
    pub async fn reload_unit(
        &self,
        name: &str,
        mode: UnitMode,
    ) -> Result<OwnedObjectPath, SystemdError> {
        Ok(self
            .get_manager_proxy()
            .reload_unit(name, &mode.to_string())
            .await?)
    }

    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub async fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        Ok(self
            .get_manager_proxy()
            .list_units()
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// May be used to get the unit object path for a unit name. It takes the unit name and returns
    /// the object path. If a unit has not been loaded yet by this name this method will fail.
    pub async fn get_unit(&self, name: &str) -> Result<OwnedObjectPath, SystemdError> {
        Ok(self.get_manager_proxy().get_unit(name).await?)
    }

    /// Returns an array of unit names and their enablement status. Note that ListUnit() returns a list of units currently loaded into memory, while ListUnitFiles() returns a list of unit
    /// files that were found on disk. Note that while most units are read directly from a unit file with the same name, some units are not backed by files and some files (templates) cannot directly be loaded
    /// as units but need to be instantiated instead.
    pub async fn list_unit_files(&self) -> Result<Vec<UnitFile>, SystemdError> {
        Ok(self
            .get_manager_proxy()
            .list_unit_files()
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    /// Returns the current enablement status of a specific unit file. The format of the string
    /// here is simply name.service, in other words, if you retrieved the unit files via
    /// list_unit_files, you may want to strip the prefix on the path to get the service name.
    pub async fn get_unit_file_state(&self, file: &str) -> Result<EnablementStatus, SystemdError> {
        Ok(self
            .get_manager_proxy()
            .get_unit_file_state(file)
            .await?
            .into())
    }

    /// May be invoked to reload all unit files.
    pub async fn reload(&self) -> Result<(), SystemdError> {
        Ok(self.get_manager_proxy().reload().await?)
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
