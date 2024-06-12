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

    /// Return adb server internal version number.
    pub async fn version(&self) -> AdbResult<u32> {
        Ok(self.execute_string("host:version").await?.parse()?)
    }

    /// Stops adb server.
    pub async fn kill(&self) -> AdbResult<()> {
        self.execute_unit("host:kill").await
    }

    /// Return devices list.
    pub async fn devices(&self) -> AdbResult<String> {
        Ok(self.execute_string("host:devices").await?)
    }

    async fn connect(&self) -> AdbResult<AdbConnection<TcpStream>> {
        AdbConnection::connect(self.addr.clone()).await
    }

    async fn execute_string(&self, command: &str) -> AdbResult<String> {
        self.connect().await?.execute_string(command).await
    }

    async fn execute_unit(&self, command: &str) -> AdbResult<()> {
        self.connect().await?.execute_unit(command).await
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new("localhost:5037")
    }
}