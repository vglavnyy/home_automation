mod devices;

use crate::devices::{SmartDevice, SmartHouse};
use crate::devices::{TempSensor, SmartSocket};

fn main() {
    let mut house = SmartHouse::new();
    {
        let temp1 = TempSensor::new(String::from("5eeecd91-1ce7-4e94-aeda-87fd76398cef"));
        let socket1 = SmartSocket::new(String::from("7cdbac61-6c94-435a-8f15-fe366e8c0b46"));
        house.add_device(String::from("kitchen"),
                         String::from("air_temp"),
                         SmartDevice::TempSensor(temp1));
        house.add_device(String::from("kitchen"),
                         String::from("air_channel_shutter"),
                         SmartDevice::SmartSocket(socket1));
    }
    {
        let temp1 = TempSensor::new(String::from("ee1d9905-a2a5-4538-a167-29394ed5ac66"));
        house.add_device(String::from("bedroom"),
                         String::from("air_temp"),
                         SmartDevice::TempSensor(temp1));
    }

    let report = house.create_report();
    println!("Report: {:?}", report);
}
