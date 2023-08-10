use crate::errors::{RecvError, RecvResult};
use crate::errors::{SendError, SendResult};
use smart_home_socket::room::Room;
use smart_home_socket::thermometer::ThermometerSession;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::{FromStr, Split};

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        let delim = " ";
        Self(s.split(delim))
    }

    pub fn next_word(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct RequestHandler {
    room: Room,
}

impl RequestHandler {
    pub fn new(room: Room) -> Self {
        Self { room }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next_word();
        match command {
            "create" => self.create(request),
            "remove" => self.remove(request),
            "switch" => self.switch(request),
            "status" => self.status(request),
            "power" => self.power(request),
            _ => format!("Bad command '{}'", command),
        }
    }

    fn create(&self, mut request: Request) -> String {
        let socket_id = request.next_word();
        let socket_id = match socket_id.is_empty() {
            false => String::from(socket_id),
            true => return "A socket id is empty!".to_string(),
        };

        match self.room.create_socket(socket_id) {
            Some(s) => format!("Created socket: {}", s),
            None => "Failed while creating socket".to_string(),
        }
    }

    fn remove(&mut self, mut request: Request) -> String {
        let socket_id = request.next_word();
        let socket_id = match socket_id.is_empty() {
            false => String::from(socket_id),
            true => return "A socket id is empty!".to_string(),
        };

        match self.room.remove_socket(socket_id) {
            Ok(s) => format!("Done: {}", s),
            Err(e) => format!("Failed: {}", e),
        }
    }

    fn switch(&mut self, mut request: Request) -> String {
        let socket_id = request.next_word();
        let socket_id = match socket_id.is_empty() {
            false => String::from(socket_id),
            true => return "A socket id is empty!".to_string(),
        };

        match self.room.switch_socket(socket_id) {
            Ok(s) => format!("Done: {}", s),
            Err(e) => format!("Failed: {}", e),
        }
    }

    fn status(&self, mut request: Request) -> String {
        let socket_id = request.next_word();
        let socket_id = match socket_id.is_empty() {
            false => String::from(socket_id),
            true => return "A socket id is empty!".to_string(),
        };

        match self.room.check_status(socket_id) {
            Ok(s) => format!("Done: {}", s),
            Err(e) => format!("Failed: {}", e),
        }
    }

    fn power(&mut self, mut request: Request) -> String {
        let socket_id = request.next_word();
        let socket_id = match socket_id.is_empty() {
            false => String::from(socket_id),
            true => return "A socket id is empty!".to_string(),
        };

        let power_value = request.next_word().trim();
        let power_value = match u32::from_str(power_value) {
            Ok(p) => p,
            Err(e) => {
                return format!("A socket power is not correct: {}", e);
            }
        };

        match self.room.change_power(socket_id, power_value) {
            Ok(s) => format!("Done: {}", s),
            Err(e) => format!("Failed: {}", e),
        }
    }
}

pub struct UdpRequestHandler {
    session: ThermometerSession,
}

impl UdpRequestHandler {
    pub fn new(session: ThermometerSession) -> Self {
        Self { session }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next_word();
        match command {
            "update" => self.update(),
            "set" => self.set(request),
            _ => format!("Bad command '{}'", command),
        }
    }

    fn update(&self) -> String {
        match self.session.update() {
            Ok(s) => format!("Done: {}", s),
            Err(e) => format!("Failed: {}", e),
        }
    }

    fn set(&mut self, mut request: Request) -> String {
        let new_value = request.next_word().trim();
        let new_value = match i32::from_str(new_value) {
            Ok(p) => p,
            Err(e) => {
                return format!("Failed while converting str to i32 {}", e);
            }
        };

        match self.session.set_value(new_value) {
            Ok(_) => format!("Successful set data {}", new_value),
            Err(e) => format!("Failed: {}", e),
        }
    }
}

pub fn send_request(stream: &mut TcpStream, str_data: String) -> SendResult {
    let output_buffer = str_data.as_bytes();
    let buffer_length = output_buffer.len() as u32;
    let result = stream.write_all(&buffer_length.to_be_bytes());
    if result.is_err() {
        let error = result.err().unwrap();
        return Err(SendError::Io(error));
    }
    match stream.write_all(output_buffer) {
        Ok(_) => Ok(str_data),
        Err(e) => Err(SendError::Io(e)),
    }
}

pub fn recv_request(stream: &mut TcpStream) -> RecvResult {
    let mut input_buffer = [0; 4];
    let _ = stream
        .read(&mut input_buffer)
        .map_err(|e| RecvError::ReadData(e.to_string()));
    let length = u32::from_be_bytes(input_buffer);

    let mut input_buffer = vec![0; length as _];
    let _ = stream
        .read(&mut input_buffer)
        .map_err(|e| RecvError::ReadData(e.to_string()));

    match String::from_utf8(input_buffer) {
        Ok(s) => Ok(s),
        Err(e) => Err(RecvError::ReadData(e.to_string())),
    }
}
