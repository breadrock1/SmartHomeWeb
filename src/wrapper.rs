use serde::{Deserialize, Serialize};
use smarthome::sockets::smart_sockets::SocketType;
use smarthome::{Room, SmartHouse, SocketTrait};

#[derive(Deserialize, Serialize)]
pub struct JsonDevice {
    name: String,
    dev_type: String,
}

impl From<&SocketType> for JsonDevice {
    fn from(value: &SocketType) -> Self {
        let name = value.name();
        let dev_type = match value {
            SocketType::Simple(d) => d.get_type(),
            SocketType::Thermometer(d) => d.get_type(),
        };

        Self { name, dev_type }
    }
}

#[derive(Deserialize, Serialize)]
pub struct JsonRoom {
    name: String,
    devices: Vec<String>,
}

impl From<&Room> for JsonRoom {
    fn from(value: &Room) -> Self {
        JsonRoom {
            name: value.get_name().to_string(),
            devices: value.get_devices().into_iter().map(|d| d.name()).collect(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct JsonSmartHouse {
    address: String,
    rooms: Vec<String>,
}

impl From<&SmartHouse> for JsonSmartHouse {
    fn from(value: &SmartHouse) -> Self {
        let address = value.get_address();

        JsonSmartHouse {
            address: address.to_string(),
            rooms: value
                .get_rooms()
                .into_iter()
                .map(|r| r.get_name().to_string())
                .collect(),
        }
    }
}
