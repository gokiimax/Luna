use std::sync::Arc;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream, sync::Mutex};

pub struct Client {
    pub socket: TcpStream,
    pub addr: String
}

impl Client {
    pub async fn read(&mut self) -> Vec<u8> {
        let mut buf = vec![0u8;1024];
        self.socket.read(&mut buf).await.unwrap();
        buf
    }

    pub async fn write(&mut self, data: &[u8]) {
        self.socket.write_all(data).await.unwrap();
    }

}

pub type ClientList = Arc<Mutex<Vec<Client>>>;