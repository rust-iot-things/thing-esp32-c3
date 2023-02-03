mod app;
mod lamp;
mod mqtt;
mod nvs_uuid;
mod observer;
mod thing;
mod thing_mqtt;
mod thing_protocol;
mod wifi;

use embedded_svc::httpd::Result;
use esp_idf_svc::{self, eventloop::EspSystemEventLoop, nvs::EspDefaultNvsPartition};

const SSID: &str = "Internet";
const PASSWORD: &str = "GibMirInternet!";

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut _wifi = wifi::connect(sysloop, nvs.clone(), SSID, PASSWORD)?;

    println!("starting app");
    app::start(&nvs).unwrap();
    Ok(())
}
