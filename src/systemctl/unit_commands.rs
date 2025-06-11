use zbus::{Connection, blocking::Connection as ConnectionBlocking};
use crate::{errors::SystemdError, systemctl::Unit};
use crate::manager::{ManagerProxyBlocking, ManagerProxy};

/// Returns an array of all currently loaded units. Note that units may be known by multiple names at the same name, and hence there might be more unit names loaded than actual units behind them. The array consists of
pub async fn list_units() -> Result<Vec<Unit>, SystemdError> {
    let connection = Connection::system().await?;
    let proxy = ManagerProxy::new(&connection).await?;
    let units = proxy.list_units().await?.iter().map(|item| {
        item.to_owned().into()
    }).collect();
    Ok(units)
}

/// Synchronous version of `list_units()`
pub fn list_units_blocking() -> Result<Vec<Unit>, SystemdError> {
    let connection = ConnectionBlocking::system()?;
    let proxy = ManagerProxyBlocking::new(&connection)?;
    let units = proxy.list_units()?.iter().map(|item| {
        item.to_owned().into()
    }).collect();
    Ok(units)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_list_units () {
        let units = list_units_blocking()
            .expect("Should be able to call list units");

        assert!(units.iter().count() > 0, "Should be able to list units with zbus connection");
    }
}

