use crate::async_client::{recv_request, send_request};
use crate::errors::SendResult;
use crate::errors::{ConnectResult, RecvResult};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct AsyncTcpClient {
    tcp: TcpStream,
}

impl AsyncTcpClient {
    pub async fn connect<T: ToSocketAddrs>(address: T) -> ConnectResult<Self> {
        let stream = TcpStream::connect(address)
            .await
            .expect("Can't establish connect");
        Ok(Self { tcp: stream })
    }

    pub async fn exec(&mut self, command: String) -> SendResult {
        send_request(&mut self.tcp, command).await
    }

    pub async fn recv_result(&mut self) -> RecvResult {
        match recv_request(&mut self.tcp).await {
            Ok(r) => Ok(format!("Result: {}", r)),
            Err(e) => Err(e),
        }
    }
}
