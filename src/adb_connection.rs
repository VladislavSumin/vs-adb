use std::str::from_utf8;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::adb_error::AdbError::AdbServerError;
use crate::adb_error::AdbResult;
use crate::adb_response_code::AdbResponseCode;
use crate::adb_response_code::AdbResponseCode::Okay;

/// Represent connection with adb server.
pub(crate) struct AdbConnection<T> where T: AsyncRead, T: AsyncWrite, T: Unpin {
    stream: T,
}

impl<T> AdbConnection<T> where T: AsyncRead, T: AsyncWrite, T: Unpin {
    pub fn new(stream: T) -> Self {
        Self { stream }
    }

    /// Encode msg and send it to adb server.
    async fn write(&mut self, msg: &str) -> AdbResult<()> {
        let hex_length = format!("{:0>4X}", msg.len() as u16);
        self.stream.write_all(hex_length.as_bytes()).await?;
        self.stream.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    /// Reads 4 bytes of input stream and cast it to [AdbResponseCode].
    async fn read_response_code(&mut self) -> AdbResult<AdbResponseCode> {
        let mut buff = [0u8; 4];
        self.stream.read_exact(&mut buff).await?;
        AdbResponseCode::try_from(&buff)
    }

    /// Read 4 bytes of data length and then read data.
    async fn read_data(&mut self) -> AdbResult<Vec<u8>> {
        let mut buff = [0u8; 4];
        self.stream.read_exact(&mut buff).await?;
        let len = usize::from_str_radix(from_utf8(buff.as_slice())?, 16)?;
        let mut vec = vec![0u8; len];
        self.stream.read_exact(vec.as_mut_slice()).await?;
        Ok(vec)
    }

    /// Read single string message from adb server.
    pub async fn read_string(&mut self) -> AdbResult<String> {
        let data = self.read_data().await?;
        Ok(from_utf8(data.as_slice())?.to_owned())
    }

    /// Execute command expect no message from adb server.
    pub async fn execute_unit(&mut self, command: &str) -> AdbResult<()> {
        self.write(command).await?;
        let response_code = self.read_response_code().await?;

        if response_code == Okay {
            Ok(())
        } else {
            let data = self.read_data().await?;
            let message = from_utf8(data.as_slice())?.to_owned();
            Err(AdbServerError(response_code, message))
        }
    }

    /// Execute command expect string response from server.
    pub async fn execute_string(&mut self, command: &str) -> AdbResult<String> {
        self.write(command).await?;
        let response_code = self.read_response_code().await?;
        let message = self.read_string().await?;

        if response_code == Okay {
            Ok(message)
        } else {
            Err(AdbServerError(response_code, message))
        }
    }
}

impl AdbConnection<TcpStream> {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> AdbResult<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(stream))
    }
}
