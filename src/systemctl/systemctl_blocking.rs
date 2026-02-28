use zbus::blocking::Connection;

use crate::{
    errors::SystemdError,
    manager::ManagerProxyBlocking,
    systemctl::{connection_level::ConnectionLevel, unit::Unit, unit_file::UnitFile},
};

pub struct SystemCtlBlocking<'a> {
    manager_proxy: Option<ManagerProxyBlocking<'a>>,
    connection_level: ConnectionLevel,
}

impl<'a> SystemCtlBlocking<'a> {
    pub fn new(connection_level: ConnectionLevel) -> Self {
        SystemCtlBlocking {
            manager_proxy: None,
            connection_level,
        }
    }

    pub fn init(&mut self) -> Result<(), SystemdError> {
        let connection = self.connection_level.get_connection_blocking()?;
        let proxy = ManagerProxyBlocking::new(&connection)?;
        self.manager_proxy = Some(proxy);
        Ok(())
    }

    /// Get access to the manager proxy directly. This allows you to call all the functions defined
    /// and documented in the ManagerProxy type, which is basically everything in
    /// org.freedesktop.systemd1.Manager. Here you will only get strings and numbers with no
    /// meaning attached to them, hopefully in future this will not be a required method.
    pub fn get_manager_proxy(&self) -> Result<&ManagerProxyBlocking<'a>, SystemdError> {
        self.manager_proxy
            .as_ref()
            .ok_or(SystemdError::InitialisationError)
    }

    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        let Some(proxy) = &self.manager_proxy else {
            return Err(SystemdError::InitialisationError);
        };
        let units = proxy.list_units()?;
        Ok(units.into_iter().map(Into::into).collect())
    }

    /// Returns an array of unit names and their enablement status. Note that ListUnit() returns a list of units currently loaded into memory, while ListUnitFiles() returns a list of unit
    /// files that were found on disk. Note that while most units are read directly from a unit file with the same name, some units are not backed by files and some files (templates) cannot directly be loaded
    /// as units but need to be instantiated instead.
    pub fn list_unit_files(&self) -> Result<Vec<UnitFile>, SystemdError> {
        let Some(proxy) = &self.manager_proxy else {
            return Err(SystemdError::InitialisationError);
        };

        let unit_files = proxy.list_unit_files()?;
        Ok(unit_files.into_iter().map(Into::into).collect())
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
    fn can_list_unit_files() {
        let mut system_ctl = SystemCtlBlocking::new(ConnectionLevel::UserLevel);

        system_ctl
            .init()
            .expect("Should be able to init connection");

        let unit_files = system_ctl.list_unit_files();

        assert!(unit_files.is_ok());

        let unit_files = unit_files.expect("Unit files should exist at this point");

        assert!(!unit_files.is_empty())
    }

    #[test]
    fn can_use_manager_proxy_directly() {
        let mut system_ctl = SystemCtlBlocking::new(ConnectionLevel::UserLevel);

        system_ctl
            .init()
            .expect("Should be able to init connection");

        let proxy = system_ctl
            .get_manager_proxy()
            .expect("Initialised at this point");

        let state = proxy.get_unit_file_state("dbus.service");

        assert!(state.is_ok());
    }

    #[test]
    fn can_list_units() {
        let mut system_ctl = SystemCtlBlocking::new(ConnectionLevel::UserLevel);

        system_ctl
            .init()
            .expect("Should be able to init connection");

        let units = system_ctl.list_units();

        assert!(units.is_ok());

        let units = units.expect("Units are OK by now");

        assert!(!units.is_empty());
    }
}
