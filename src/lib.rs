pub mod manager;

pub use manager::ManagerProxy;
pub use zbus::Connection;


#[cfg(test)]
mod tests {
    use std::error::Error;
    use zbus::Connection;
    use crate::manager::ManagerProxy;

    #[test]
    fn can_list_units() {
        let res: Result<(), Box<dyn Error>> = smol::block_on(async {
            let connection = Connection::system().await?;
            let proxy = ManagerProxy::new(&connection).await?;
            let res = proxy.list_units().await?;

            assert!(res.len() > 0);
            Ok(())
        });

        assert!(res.is_ok());
    }
}
