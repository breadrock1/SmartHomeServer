use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use smart_home_server::errors::*;
use smart_home_server::handlers::*;
use smart_home_socket::room::*;

fn main() {
    let address = String::from("127.0.0.1:55123");
    let server = TcpListener::bind(address).expect("Failed while binding TcpListener");

    for connection in server.incoming() {
        let connection = match connection {
            Ok(c) => c,
            Err(e) => {
                println!("Can't establish connection: {}", e);
                continue;
            }
        };

        let peer_address = match connection.peer_addr() {
            Ok(a) => a.to_string(),
            Err(_) => "unknown".into(),
        };

        println!("New client connected: {}", peer_address);

        let room = Room::default();
        thread::spawn(move || {
            if handle_connection(connection, room).is_err() {
                println!("Client disconnected: {}", peer_address);
            }
        });
    }

    fn handle_connection(mut connection: TcpStream, room: Room) -> Result<(), ConnectError> {
        let mut handler = RequestHandler::new(room);
        loop {
            let input_string = match recv_request(&mut connection) {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed while reading request: {}", e);
                    return Err(ConnectError::Other(e.to_string()));
                }
            };

            let request = Request::new(input_string.as_str());
            let handler_status = handler.handle(request);
            match send_request(&mut connection, handler_status) {
                Ok(s) => println!("{}", s),
                Err(e) => {
                    println!("Failed while writing response: {}", e);
                    return Err(ConnectError::Other(e.to_string()));
                }
            };
        }
    }
}
