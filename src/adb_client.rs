use tokio::net::TcpStream;
use crate::adb_connection::AdbConnection;
use crate::adb_error::AdbResult;

pub struct AdbClient {
    addr: String,
}


impl AdbClient {
    pub fn new<T: ToString>(addr: T) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    /// Return adb server internal version number
    pub async fn get_server_version(&self) -> AdbResult<u32> {
        let mut connection = self.connect().await?;
        connection.write("host:version").await?;
        Ok(connection.read().await?.parse()?)
    }

    async fn connect(&self) -> AdbResult<AdbConnection<TcpStream>> {
        AdbConnection::connect(self.addr.clone()).await
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new("localhost:5037")
    }
}