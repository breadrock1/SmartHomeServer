use std::ops::Not;

#[derive(Default, Clone)]
pub struct Socket {
    pub identify: String,
    pub power: u32,
    pub switch: bool,
}

impl Socket {
    pub fn new(identify: String) -> Self {
        Socket {
            identify,
            power: 80,
            switch: false,
        }
    }

    pub fn switch_socket(&mut self) {
        let new_value = &self.switch.not();
        self.switch = *new_value;
    }

    pub fn set_power(&mut self, power: u32) {
        self.power = power;
    }

    pub fn get_status(&self) -> String {
        format!(
            "Socket: {}\nStatus: {}\nPower: {}\n",
            self.identify, self.switch, self.power
        )
    }
}

#[derive(Default, Clone)]
pub struct Thermometer {
    temperature: i32,
}

impl Thermometer {
    pub fn new(temperature: i32) -> Self {
        Self { temperature }
    }

    pub fn update(&mut self, temperature: i32) {
        self.temperature = temperature
    }

    pub fn get_status(&self) -> String {
        format!("Thermometer value: {}", self.temperature)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_socket() {
        let socket_name = "socket_1";
        let mut socket = Socket::new(String::from(socket_name));

        assert_eq!(socket.identify.as_str(), socket_name);
        assert!(!socket.switch);
        assert_eq!(socket.power, 80);

        let _ = &socket.switch_socket();
        assert!(socket.switch);

        let _ = &socket.set_power(90);
        assert_eq!(socket.power, 90);

        let status = socket.get_status();
        let cmp_status = format!(
            "Socket: {}\nStatus: {}\nPower: {}\n",
            &socket_name, true, 90
        );

        assert_eq!(status, cmp_status);
    }

    #[test]
    fn create_thermometer() {
        let mut thermometer = Thermometer::new(80);
        assert_eq!(thermometer.temperature, 80);

        thermometer.update(90);
        assert_eq!(thermometer.temperature, 90);

        let cmp_status = format!("Thermometer value: {}", 90);
        assert_eq!(cmp_status, thermometer.get_status());
    }
}
