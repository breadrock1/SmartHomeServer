use smart_home_server::errors::*;
use smart_home_server::handlers::*;
use std::net::UdpSocket;
use std::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    tcp: TcpStream,
}

impl Client {
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

pub struct UdpClient {
    pub udp: UdpSocket,
}

impl UdpClient {
    pub fn connect<T: ToSocketAddrs>(address: T) -> ConnectResult<Self> {
        let stream = UdpSocket::bind(address).expect("Can't establish connect");
        Ok(Self { udp: stream })
    }

    pub fn send(&mut self, value: i32) {
        let msg_str = value.to_string();
        let msg_bytes = msg_str.as_bytes();
        let _ = self.udp.send(msg_bytes);
    }
}
