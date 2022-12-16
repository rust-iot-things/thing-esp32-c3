use std::net::Ipv4Addr;
use std::time::Duration;

use embedded_svc::wifi::*;
use esp_idf_hal::modem::WifiModem;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::netif::{EspNetifWait, EspNetif};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{EspWifi, WifiWait};

use embedded_svc::httpd::Result;

pub fn connect<'a>(
    sysloop: EspSystemEventLoop,
    nvs: EspDefaultNvsPartition,
    ssid: &'a str,
    password: &'a str,
) -> Result<Box<EspWifi<'a>>> {
    unsafe {
        let modem: WifiModem = WifiModem::new();
        let mut wifi = Box::new(EspWifi::new(modem, sysloop.clone(), Some(nvs))?);

        println!("Wifi created, about to scan");

        let ap_printlns = wifi.scan()?;

        let ours = ap_printlns.into_iter().find(|a| a.ssid == ssid);

        let channel = if let Some(ours) = ours {
            println!(
                "Found configured access point {} on channel {}",
                ssid, ours.channel
            );
            Some(ours.channel)
        } else {
            println!(
                "Configured access point {} not found during scanning, will go with unknown channel",
                ssid
            );
            None
        };

        wifi.set_configuration(&Configuration::Mixed(
            ClientConfiguration {
                ssid: ssid.into(),
                password: password.into(),
                channel,
                ..Default::default()
            },
            AccessPointConfiguration {
                ssid: "aptest".into(),
                channel: channel.unwrap_or(1),
                ..Default::default()
            },
        ))?;


        wifi.start()?;

        println!("Starting wifi...");
    
        if !WifiWait::new(&sysloop)?
            .wait_with_timeout(Duration::from_secs(20), || wifi.is_started().unwrap())
        {
            println!("Wifi did not start");
        }
    
        println!("Connecting wifi...");
    
        wifi.connect()?;
    
        if !EspNetifWait::new::<EspNetif>(wifi.sta_netif(), &sysloop)?.wait_with_timeout(
            Duration::from_secs(20),
            || {
                wifi.is_connected().unwrap()
                    && wifi.sta_netif().get_ip_info().unwrap().ip != Ipv4Addr::new(0, 0, 0, 0)
            },
        ) {
            println!("Wifi did not connect or did not receive a DHCP lease");
        }
    
        let ip_println = wifi.sta_netif().get_ip_info()?;
    
        println!("Wifi DHCP println: {:?}", ip_println);
    
        Ok(wifi)
    }
}
