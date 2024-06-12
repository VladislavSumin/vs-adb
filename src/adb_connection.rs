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
    pub async fn write(&mut self, msg: &str) -> AdbResult<()> {
        let hex_length = format!("{:0>4X}", u16::try_from(msg.len()).unwrap());
        self.stream.write_all(hex_length.as_bytes()).await?;
        self.stream.write_all(msg.as_bytes()).await?;
        Ok(())
    }

    /// TODO write documentation
    pub async fn read(&mut self) -> AdbResult<String> {
        let mut buff = [0u8; 4];

        // Read and check response code
        self.stream.read_exact(&mut buff).await?;
        let response_code = AdbResponseCode::try_from(&buff)?;

        // Read data len
        self.stream.read_exact(&mut buff).await?;
        let len = usize::from_str_radix(from_utf8(buff.as_slice())?, 16)?;

        // Read message
        let mut buff = vec![];
        self.stream.read_to_end(&mut buff).await?;
        let message = from_utf8(buff.as_slice())?.to_owned();

        // TODO temp check
        assert_eq!(message.len(), len);

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
