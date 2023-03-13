use std::error::Error;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, HashSet};
use hidapi::{HidApi, HidDevice};
use crate::unix_socket::UnixSocket;

pub struct KeyEvent {
    pub device_path: String,
    pub key_code: u16,
    pub key_state: u8,
    pub timestamp: u64,
}

pub struct KeyMonitor {
    api: HidApi,
    socket: UnixSocket,
    last_state: HashMap<String, u8>,
}

impl KeyMonitor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let api = HidApi::new()?;
        let socket = UnixSocket::new("/tmp/key_events.sock")?;
        let last_state = HashMap::new();
        Ok(KeyMonitor { api, socket, last_state })
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let devices = self.api.device_list();
            let keyboard_devices = devices
                .into_iter()
                .filter(|d| d.interface_number() == 1 && d.usage_page() == 0x01 && d.usage() == 0x06);
            for device in keyboard_devices {
                let events = self.get_device_keyboard_events(&device);
                for event in events {
                    let device_path = format!("DevSrvsID:{}", device.get_serial_number_string().unwrap_or_default());
                    let state_key = format!("{}-{}", device_path, event.0);
                    if let Some(last_key_state) = self.last_state.get_mut(&state_key) {
                        if *last_key_state != event.1 {
                            let key_event = KeyEvent {
                                device_path: device_path.clone(),
                                key_code: event.0,
                                key_state: event.1,
                                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                            };
                            self.socket.send_message(&key_event)?;
                            *last_key_state = event.1;
                        }
                    } else {
                        self.last_state.insert(state_key, event.1);
                    }
                }
            }
            thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn get_device_keyboard_events(&self, device: &HidDevice) -> Vec<(u16, u8)> {
        let mut buf = [0u8; 8];
        match device.open_device(&self.api) {
            Ok(dev) => {
                let mut events = Vec::new();
                loop {
                    // Add a timeout to the read method to prevent it from blocking indefinitely
                    if let Ok(read_size) = dev.read_timeout(&mut buf, std::time::Duration::from_millis(10)) {
                        if read_size != 8 {
                            continue;
                        }
                        let key_code = buf[2] as u16 | ((buf[3] as u16) << 8);
                        let key_state = if buf[0] & 0x01 == 0x01 { 1 } else { 0 };
                        events.push((key_code, key_state));
                    } else {
                        break;
                    }
                }
                events
            }
            Err(_) => Vec::new(),
        }
    }
}

