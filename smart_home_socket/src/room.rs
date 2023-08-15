use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::sync::Arc;

use crate::errors::*;
use crate::socket::*;

#[derive(Default, Clone)]
pub struct Room {
    sockets: Arc<DashMap<String, Socket>>,
}

impl Room {
    pub fn create_socket(&self, socket_id: String) -> Option<String> {
        let socket_entry = self.sockets.entry(socket_id.clone());
        match socket_entry {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => {
                let socket = Socket::new(socket_id.clone());
                v.insert(socket);
                Some(socket_id)
            }
        }
    }

    pub fn switch_socket(&self, socket_id: String) -> SocketResult {
        let socket_id_str = socket_id.as_str();
        let socket_opt = self.sockets.get_mut(socket_id_str);
        match socket_opt {
            Some(mut s) => {
                s.switch_socket();
                Ok(format!(
                    "Socket {} has been switched {}",
                    s.identify, s.switch
                ))
            }
            None => {
                let msg = "There is no spcket with passed name";
                Err(SocketError::SwitchingError(msg.to_string()))
            }
        }
    }

    pub fn append_socket(&mut self, socket: Socket) -> SocketResult {
        let socket_name = socket.identify.clone();
        let socket_entry = self.sockets.entry(socket_name.clone());
        match socket_entry {
            Entry::Occupied(_) => {
                let msg = "Failed while appending socket";
                Err(SocketError::AppendingError(msg.to_string()))
            }
            Entry::Vacant(v) => {
                v.insert(socket);
                Ok(format!("Appended socket {}", socket_name))
            }
        }
    }

    pub fn remove_socket(&mut self, socket_id: String) -> SocketResult {
        let socket_entry = self.sockets.entry(socket_id.clone());
        match socket_entry {
            Entry::Occupied(o) => {
                o.remove();
                Ok(format!("Removed socket {}", socket_id))
            }
            Entry::Vacant(_) => {
                let msg = "Failed while removing socket";
                Err(SocketError::RemovingError(msg.to_string()))
            }
        }
    }

    pub fn check_status(&self, socket_id: String) -> SocketResult {
        let socket_entry = self.sockets.entry(socket_id);
        match socket_entry {
            Entry::Occupied(o) => {
                let status = o.get().get_status();
                Ok(status)
            }
            Entry::Vacant(_) => {
                let msg = "Failed while getting socket status";
                Err(SocketError::StatusError(msg.to_string()))
            }
        }
    }

    pub fn change_power(&mut self, socket_id: String, power: u32) -> SocketResult {
        let socket_id_str = socket_id.as_str();
        let socket_opt = self.sockets.get_mut(socket_id_str);
        match socket_opt {
            Some(mut s) => {
                s.set_power(power);
                Ok(format!("Set power {} for socket: {}", s.power, s.identify))
            }
            None => {
                let msg = "Can't find socket by passed id";
                Err(SocketError::PowerError(msg.to_string()))
            }
        }
    }

    pub fn sockets_iter(&self) -> impl Iterator<Item = String> {
        let socket_names: Vec<String> = self
            .sockets
            .as_ref()
            .clone()
            .into_iter()
            .map(|(l, _)| l)
            .collect();

        socket_names.into_iter()
    }

    pub fn get_socket(&self, name: String) -> Option<Socket> {
        match self.sockets.entry(name) {
            Entry::Occupied(s) => Some(s.get().clone()),
            Entry::Vacant(_) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_room() {
        let mut room = Room::default();
        let socket_1 = "socket_1".into();
        let _socket_id_1 = room
            .create_socket(socket_1)
            .expect("Failed while creating socket");

        let socket_2 = Socket::new("socket_2".to_string());
        let _socket_id_2 = room
            .append_socket(socket_2)
            .expect("Failed while appending socket");

        assert_eq!(room.sockets.len(), 2);

        let remove_result = room.remove_socket("socket_1".to_string());
        assert!(remove_result.is_ok());
    }
}
