mod mqtt;
mod observer;
mod thing;
mod thing_mqtt;
mod thing_protocol;
mod wifi;

use embedded_svc::httpd::Result;
use esp_idf_svc::{self, eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

use std::{thread, time::Duration};

const SSID: &str = "YAKINDU";
const PASSWORD: &str = "internet";

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut _wifi = wifi::connect(sysloop, nvs, SSID, PASSWORD)?;

    let _app = thing_mqtt::ThingMQTT::new();
    println!("app started");
    loop {
        thread::sleep(Duration::from_secs(1));
        if false {
            break;
        }
    }
    Ok(())
}
