use rand::{thread_rng, Rng};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let address = String::from("[::]:2000");
    let server = UdpSocket::bind(address).unwrap();
    let duration = Duration::new(5, 0);
    server.set_read_timeout(Some(duration)).unwrap();

    let f_th = thread::spawn(move || launch_thermometer());
    let s_th = thread::spawn(move || bind_udp_datagrams(server));

    let _ = f_th.join();
    let _ = s_th.join();
}

fn bind_udp_datagrams(server: UdpSocket) {
    let mut buf = [0, 128];
    let timeout = Duration::from_secs(2);

    loop {
        if let Ok((l, a)) = server.recv_from(&mut buf) {
            let received_str = String::from_utf8_lossy(&buf);
            println!("{} bytes receiver from {}", l, a);
            println!("Received data {}", received_str);
        };

        thread::sleep(timeout);
    }
}

fn launch_thermometer() {
    let bind_address = "[::]:0";
    let udp_client = UdpSocket::bind(bind_address).expect("Can't establish connect");

    let mut rng = thread_rng();
    let target_address = "localhost:2000";
    let timeout = Duration::from_secs(2);

    loop {
        let gen_temp_value: i32 = rng.gen_range(20..300);
        let gen_temp_str = &gen_temp_value.to_string();
        if let Err(e) = udp_client.send_to(gen_temp_str.as_bytes(), target_address) {
            println!("Failed while sending datagram: {}", e);
        };

        thread::sleep(timeout);
    }
}
