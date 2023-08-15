use crate::errors::SendResult;
use crate::errors::{ConnectResult, RecvResult};
use crate::{recv_request, send_request};
use std::net::{TcpStream, ToSocketAddrs};

pub struct TcpClient {
    tcp: TcpStream,
}

impl TcpClient {
    pub fn connect<T: ToSocketAddrs>(address: T) -> ConnectResult<Self> {
        let stream = TcpStream::connect(address).expect("Can't establish connect");
        Ok(Self { tcp: stream })
    }

    pub fn exec(&mut self, command: String) -> SendResult {
        send_request(&mut self.tcp, command)
    }

    pub fn recv_result(&mut self) -> RecvResult {
        match recv_request(&mut self.tcp) {
            Ok(r) => Ok(format!("Result: {}", r)),
            Err(e) => Err(e),
        }
    }
}
