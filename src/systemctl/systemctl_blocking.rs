use crate::{
    errors::SystemdError,
    manager::ManagerProxyBlocking,
    systemctl::{connection_level::ConnectionLevel, unit::Unit},
};

pub struct SystemCtlBlocking<'a> {
    manager_proxy: Option<ManagerProxyBlocking<'a>>,
    connection_level: ConnectionLevel,
}

impl<'a> SystemCtlBlocking<'a> {
    pub fn new(connection_level: ConnectionLevel) -> Self {
        SystemCtlBlocking {
            manager_proxy: None,
            connection_level: connection_level,
        }
    }
    /// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them.
    pub fn list_units(&self) -> Result<Vec<Unit>, SystemdError> {
        let Some(proxy) = &self.manager_proxy else {
            return Err(SystemdError::InitialisationError);
        };
        let units = proxy.list_units()?;
        Ok(units.into_iter().map(Into::into).collect())
    }

    pub fn init(&mut self) -> Result<(), SystemdError> {
        let connection = self.connection_level.get_connection_blocking()?;
        let proxy = ManagerProxyBlocking::new(&connection)?;
        self.manager_proxy = Some(proxy);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_list_units() {
        let mut system_ctl = SystemCtlBlocking::new(ConnectionLevel::UserLevel);

        system_ctl
            .init()
            .expect("Should be able to init connection");

        let units = system_ctl.list_units();

        assert!(units.is_ok());

        let units = units.expect("Units are OK by now");

        assert!(units.len() > 0);
    }
}
