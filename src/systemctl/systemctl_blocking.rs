use zbus::blocking::Connection;

use crate::{
    errors::SystemdError,
    manager::ManagerProxyBlocking,
    systemctl::{
        connection_level::ConnectionLevel,
        unit::Unit,
        unit_file::{EnablementStatus, UnitFile},
    },
};

pub struct SystemCtlBlockingBuilder {
    connection_level: ConnectionLevel,
}

impl Default for SystemCtlBlockingBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemCtlBlockingBuilder {
    pub fn new() -> Self {
        Self {
            connection_level: ConnectionLevel::UserLevel,
        }
    }

    pub fn with_system_connection_level(mut self) -> Self {
        self.connection_level = ConnectionLevel::SystemLevel;
        self
    }

    pub fn init<'a>(self) -> Result<SystemCtlBlocking<'a>, SystemdError> {
        let connection = self.connection_level.get_connection_blocking()?;
        let proxy = ManagerProxyBlocking::new(&connection)?;
        Ok(SystemCtlBlocking {
            manager_proxy: proxy,
            connection_level: self.connection_level,
        })
    }
}

pub struct SystemCtlBlocking<'a> {
    manager_proxy: ManagerProxyBlocking<'a>,
    #[allow(unused)]
    connection_level: ConnectionLevel,
}

impl<'a> SystemCtlBlocking<'a> {
    /// Get access to the manager proxy directly. This allows you to call all the functions defined
    /// and documented in the ManagerProxy type, which is basically everything in
    /// org.freedesktop.systemd1.Manager. Here you will only get strings and numbers with no
    /// meaning attached to them, hopefully in future this will not be a required method.
    pub fn get_manager_proxy(&self) -> &ManagerProxyBlocking<'a> {
        &self.manager_proxy
    }

    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        let proxy = self.get_manager_proxy();

        let units = proxy.list_units()?;
        Ok(units.into_iter().map(Into::into).collect())
    }

    /// May be used to get the unit object path for a unit name. It takes the unit name and returns
    /// the object path. If a unit has not been loaded yet by this name this method will fail.
    pub fn get_unit(&self, name: &str) -> Result<String, SystemdError> {
        let proxy = self.get_manager_proxy();
        let owned_object_path = proxy.get_unit(name)?;

        Ok(owned_object_path.to_string())
    }

    /// Returns an array of unit names and their enablement status. Note that ListUnit() returns a list of units currently loaded into memory, while ListUnitFiles() returns a list of unit
    /// files that were found on disk. Note that while most units are read directly from a unit file with the same name, some units are not backed by files and some files (templates) cannot directly be loaded
    /// as units but need to be instantiated instead.
    pub fn list_unit_files(&self) -> Result<Vec<UnitFile>, SystemdError> {
        let proxy = self.get_manager_proxy();

        let unit_files = proxy.list_unit_files()?;
        Ok(unit_files.into_iter().map(Into::into).collect())
    }

    /// Returns the current enablement status of a specific unit file. The format of the string
    /// here is simply name.service, in other words, if you retrieved the unit files via
    /// list_unit_files, you may want to strip the prefix on the path to get the service name.
    pub fn get_unit_file_state(&self, file: &str) -> Result<EnablementStatus, SystemdError> {
        let proxy = self.get_manager_proxy();
        let unit_file_state = proxy.get_unit_file_state(file)?;

        Ok(unit_file_state.into())
    }

    /// May be invoked to reload all unit files.
    pub fn reload(&self) -> Result<(), SystemdError> {
        let proxy = self.get_manager_proxy();
        Ok(proxy.reload()?)
    }
}

impl ConnectionLevel {
    fn get_connection_blocking(&self) -> Result<Connection, SystemdError> {
        let connection = match self {
            ConnectionLevel::UserLevel => Connection::session()?,
            ConnectionLevel::SystemLevel => Connection::system()?,
        };

        Ok(connection)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_unit() {
        let system_ctl_builder = SystemCtlBlockingBuilder::new();

        let system_ctl = system_ctl_builder
            .init()
            .expect("Should be able to init connection");

        let unit = system_ctl.get_unit("dbus.service");

        assert!(unit.is_ok());

        let units = system_ctl
            .list_units()
            .expect("Should be able to list units");

        for unit in units {
            let unit = system_ctl.get_unit(&unit.name);
            dbg!(&unit);

            assert!(unit.is_ok());
        }
    }

    #[test]
    fn can_get_valid_unit_file_state() {
        let system_ctl_builder = SystemCtlBlockingBuilder::new();

        let system_ctl = system_ctl_builder
            .init()
            .expect("Should be able to init connection");

        let units = system_ctl
            .list_unit_files()
            .expect("Should be able to list units");

        for unit in units {
            let file = unit
                .path
                .split('/')
                .next_back()
                .expect("Should not be empty string");
            let status = system_ctl
                .get_unit_file_state(file)
                .expect("Should be able to get status");

            if let EnablementStatus::Other(_) = status {
                panic!("All unit files should have a valid status returned");
            };
        }
    }

    #[test]
    fn can_list_unit_files() {
        let system_ctl_builder = SystemCtlBlockingBuilder::new();
        let system_ctl = system_ctl_builder
            .init()
            .expect("Should be able to init connection");

        let unit_files = system_ctl.list_unit_files();

        assert!(unit_files.is_ok());

        let unit_files = unit_files.expect("Unit files should exist at this point");

        assert!(!unit_files.is_empty())
    }

    #[test]
    fn can_use_manager_proxy_directly() {
        let system_ctl_builder = SystemCtlBlockingBuilder::new();
        let system_ctl = system_ctl_builder
            .init()
            .expect("Should be able to init connection");

        let proxy = system_ctl.get_manager_proxy();

        let state = proxy.get_unit_file_state("dbus.service");

        assert!(state.is_ok());
    }

    #[test]
    fn can_list_units() {
        let system_ctl_builder = SystemCtlBlockingBuilder::new();
        let system_ctl = system_ctl_builder
            .init()
            .expect("Should be able to init connection");

        let units = system_ctl.list_units();

        assert!(units.is_ok());

        let units = units.expect("Units are OK by now");

        assert!(!units.is_empty());
    }
}
