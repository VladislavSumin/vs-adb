use tokio::net::TcpStream;
use crate::adb_connection::AdbConnection;
use crate::adb_error::AdbResult;

mod adb_response_code;
mod adb_error;
mod adb_connection;

#[tokio::main]
async fn main() {
    let client = AdbClient::default();

    let mut conn = client.connect().await.unwrap();
    conn.write("host:version2").await.unwrap();
    conn.read().await.unwrap();
}

struct AdbClient {
    addr: String,
}


impl AdbClient {
    fn new<T: ToString>(addr: T) -> Self {
        Self {
            addr: addr.to_string(),
        }
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
