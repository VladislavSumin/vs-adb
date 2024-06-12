use std::str::from_utf8;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};
use crate::adb_error::AdbResult;
use crate::adb_response_code::AdbResponseCode;

pub(crate) struct AdbConnection<T> where T: AsyncRead, T: AsyncWrite, T: Unpin {
    stream: T,
}

impl<T> AdbConnection<T> where T: AsyncRead, T: AsyncWrite, T: Unpin {
    pub fn new(stream: T) -> Self {
        Self { stream }
    }

    pub async fn write(&mut self, msg: &str) -> AdbResult<()> {
        self.stream.write_all(Self::encode_message(msg).as_bytes()).await?;
        Ok(())
    }

    pub async fn read(&mut self) -> AdbResult<String> {
        let mut buff = [0u8; 4];

        // Read and check response code
        self.stream.read_exact(&mut buff).await?;
        let response_code = AdbResponseCode::try_from(&buff)?;

        println!("Response code: {:#?}", response_code);

        let len = self.read_length().await?;

        print!("Response len: {}", len);

        let mut buff = vec![];
        self.stream.read_to_end(&mut buff).await?;
        let message = from_utf8(buff.as_slice())?;
        println!("Message len = {}", message.len());
        println!("Message: {}", message);

        Ok("".to_owned())
    }

    async fn read_length(&mut self) -> AdbResult<usize> {
        let mut buff: [u8; 4] = [0; 4];
        self.stream.read_exact(&mut buff).await?;

        let response = from_utf8(&buff)?;

        Ok(usize::from_str_radix(response, 16)?)
    }

    fn encode_message(msg: &str) -> String {
        let hex_length = format!("{:0>4X}", u16::try_from(msg.len()).unwrap());
        format!("{}{}", hex_length, msg)
    }
}

impl AdbConnection<TcpStream> {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> AdbResult<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self::new(stream))
    }
}