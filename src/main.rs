#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

struct AdbClient {
    host: String,
    port: u16,
}

impl AdbClient {
    fn new<T: ToString>(host: T, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
        }
    }
}

impl Default for AdbClient {
    fn default() -> Self {
        Self::new("localhost", 5037)
    }
}
