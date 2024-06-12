use crate::adb_client::AdbClient;

mod adb_response_code;
mod adb_error;
mod adb_connection;
mod adb_client;

#[tokio::main]
async fn main() {
    let client = AdbClient::default();

    let version = client.get_server_version().await.unwrap();
    println!("Adb server version: {version}");
}
