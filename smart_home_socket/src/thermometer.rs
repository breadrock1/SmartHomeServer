use std::sync::{Arc, RwLock};

use crate::errors::*;
use crate::socket::*;

#[derive(Default, Clone)]
pub struct ThermometerSession {
    thermometer: Arc<RwLock<Thermometer>>,
}

impl ThermometerSession {
    pub fn update(&self) -> ThermometerResult {
        match self.thermometer.read() {
            Ok(t) => Ok(t.get_status()),
            Err(_) => {
                let msg = "Failed while getting status";
                Err(ThermometerError::UpdateError(msg.to_string()))
            }
        }
    }

    pub fn set_value(&mut self, value: i32) -> UdpResult {
        match self.thermometer.write() {
            Ok(mut t) => {
                t.update(value);
                Ok(())
            }
            Err(_) => {
                let msg = "Failed while setting status";
                Err(ThermometerError::SetupError(msg.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_session() {
        let mut session = ThermometerSession::default();
        let _ = session.set_value(30);
        let result = session.update().unwrap();
        assert_eq!(result, format!("Thermometer value: {}", 30));

        let _ = session.set_value(50);
        let result = session.update().unwrap();
        assert_eq!(result, format!("Thermometer value: {}", 50));
    }
}
