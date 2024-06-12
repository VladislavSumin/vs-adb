mod adb_response_code;
mod adb_error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::adb_error::AdbResult;
use crate::adb_response_code::AdbResponseCode;


#[tokio::main]
async fn main() {
    let client = AdbClient::default();

    let mut conn = client.connect().await.unwrap();
    conn.write("host:version").await.unwrap();
    conn.read().await.unwrap();
}

struct AdbClient {
    addr: String,
}

struct AdbConnection {
    stream: TcpStream,
}

impl AdbConnection {
    async fn connect<A: ToSocketAddrs>(addr: A) -> AdbResult<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { stream })
    }

    async fn write(&mut self, msg: &str) -> AdbResult<()> {
        self.stream.write_all(Self::encode_message(msg).as_bytes()).await?;
        Ok(())
    }

    async fn read(&mut self) -> AdbResult<String> {
        let mut buff = [0u8; 4];

        // Read and check response code
        self.stream.read_exact(&mut buff).await?;
        let response_code = AdbResponseCode::try_from(&buff)?;

        println!("Response code: {:#?}", response_code);

        Ok("".to_owned())
    }

    fn encode_message(msg: &str) -> String {
        let hex_length = format!("{:0>4X}", u16::try_from(msg.len()).unwrap());
        format!("{}{}", hex_length, msg)
    }
}

impl AdbClient {
    fn new<T: ToString>(addr: T) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    async fn connect(&self) -> AdbResult<AdbConnection> {
        AdbConnection::connect(self.addr.clone()).await
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new("localhost:5037")
    }
}
