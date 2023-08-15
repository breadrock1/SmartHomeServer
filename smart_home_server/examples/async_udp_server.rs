use smart_home_server::errors::HandleResult;
use std::thread::sleep;
use std::time::Duration;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let address = String::from("[::]:2000");
    let server = UdpSocket::bind(address).await.unwrap();

    tokio::spawn(async move {
        let _ = launch_thermometer().await;
    });
    tokio::spawn(async move {
        let _ = bind_udp_datagrams(server).await;
    });
}

async fn bind_udp_datagrams(server: UdpSocket) -> HandleResult {
    let mut buf = [0, 128];
    let timeout = Duration::from_secs(2);

    loop {
        if let Ok((l, a)) = server.recv_from(&mut buf).await {
            let received_str = String::from_utf8_lossy(&buf);
            println!("{} bytes receiver from {}", l, a);
            println!("Received data {}", received_str);
        };

        sleep(timeout);
    }
}

async fn launch_thermometer() -> HandleResult {
    let bind_address = "[::]:0";
    let udp_client = UdpSocket::bind(bind_address)
        .await
        .expect("Can't establish connect");

    let target_address = "localhost:2000";
    let timeout = Duration::from_secs(2);

    loop {
        let rng = rand::random::<i32>();
        let gen_temp_str = &rng.to_string();
        if let Err(e) = udp_client
            .send_to(gen_temp_str.as_bytes(), target_address)
            .await
        {
            println!("Failed while sending datagram: {}", e);
        };

        sleep(timeout);
    }
}
