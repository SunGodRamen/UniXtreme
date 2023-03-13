use hidapi::HidApi;
use hidapi::HidDeviceInfo;
use std::error::Error;

pub struct HidDevice {
    device_info: HidDeviceInfo,
}

impl HidDevice {
    pub fn new(device_info: HidDeviceInfo) -> Self {
        HidDevice { device_info }
    }

    pub fn open(&self, api: &HidApi) -> Result<hidapi::HidDevice, String> {
        api.open(self.device_info.vendor_id(), self.device_info.product_id())
            .map_err(|e| format!("Error opening device: {}", e))
    }

    pub fn close(&self, device: &mut hidapi::HidDevice) {
        device.close_device();
    }

    pub fn read_input_report(&self, device: &mut hidapi::HidDevice, report_id: u8, report_size: usize) -> Result<Vec<u8>, String> {
        let mut report = vec![0u8; report_size];
        report[0] = report_id;
        let read_size = device.read(&mut report)?;
        if read_size != report_size {
            Err(format!("Unexpected report size. Expected {}, got {}", report_size, read_size))
        } else {
            Ok(report)
        }
    }

    pub fn serial_number(&self) -> Option<String> {
        self.device_info.get_serial_number_string().ok()
    }

    pub fn path(&self) -> String {
        self.device_info.path().to_str().unwrap().to_string()
    }
}

