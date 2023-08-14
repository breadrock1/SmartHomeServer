use smart_home_server::errors::*;
use smart_home_server::handlers::*;
use smart_home_socket::room::*;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let address = String::from("127.0.0.1:55123");
    let server = TcpListener::bind(address)
        .await
        .expect("Failed while binding TcpListener");

    loop {
        let (connection, _) = match server.accept().await {
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
        tokio::spawn(async move {
            if handle_connection(connection, room).await.is_err() {
                println!("Client disconnected: {}", peer_address);
            }
        });
    }

    async fn handle_connection(mut connection: TcpStream, room: Room) -> HandleResult {
        let mut handler = RequestHandler::new(room);
        loop {
            let input_string = match recv_request(&mut connection).await {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed while reading request: {}", e);
                    return Err(ConnectError::Other(e.to_string()));
                }
            };

            let request = Request::new(input_string.as_str());
            let handler_status = handler.handle(request);
            match send_request(&mut connection, handler_status).await {
                Ok(s) => println!("{}", s),
                Err(e) => {
                    println!("Failed while writing response: {}", e);
                    return Err(ConnectError::Other(e.to_string()));
                }
            };
        }
    }
}
