use std::process::Command;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct AdbDevices {
    mac_address: String,
    devices_name: String,
    device_product: String,
    model: String,
    transport_id: i32,
}

impl AdbDevices {
    pub fn new(message: &str) -> AdbDevices {
        let adb_split_message: Vec<&str> = message.trim().split_whitespace().collect();

        println!("adb message len : {}", adb_split_message.len());

        let devices_name = adb_split_message
            .get(4)
            .expect("Get Devices Name Error")
            .replace("device:", "");

        let model = adb_split_message
            .get(3)
            .expect("Get model Error")
            .replace("model:", "");

        let mac_address = adb_split_message
            .get(0)
            .expect("Get mac_address Error")
            .to_string();

        let device_product = adb_split_message
            .get(2)
            .expect("Get device_product Error")
            .replace("product:", "");

        let transport_id = adb_split_message
            .get(5)
            .expect("Get transport_id Error")
            .replace("transport_id:", "")
            .parse::<i32>()
            .expect("Parse transport id Error");

        AdbDevices {
            devices_name: devices_name,
            mac_address: mac_address,
            device_product: device_product,
            model: model,
            transport_id: transport_id,
        }
    }

    
}

#[tauri::command]
pub fn list_devices() -> Vec<String> {

    let mut devices_list: Vec<String> = Vec::new();

    let x = Command::new("E:\\project\\project\\project\\MaaG\\src-tauri\\source\\windows\\adb")
        .args(["devices", "-l"])
        .output()
        .expect("No Found Adb Connection");

    match String::from_utf8(x.stdout) {
        Ok(x) => {
            let adb_command_back_message: Vec<&str> = x.split("\n").collect();
            for i in 1..(adb_command_back_message.len() - 2) {
                let adb_link: &str = adb_command_back_message.get(i).expect("error");
                let mut a = AdbDevices::new(adb_link);
                let b = serde_json::to_string(&a).expect("parse");
                println!("{}", b);
                devices_list.push(b);
            }
        }
        Err(_) => {
            println!("String Error")
        }
    }


    return devices_list;
}
