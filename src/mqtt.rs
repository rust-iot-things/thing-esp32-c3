#![allow(unused_imports)]
use anyhow::bail;
use embedded_svc::mqtt::client::Connection;
use embedded_svc::utils::mqtt::client::ConnState;
use esp_idf_svc::tls::X509;
use include_cstr::include_cstr;

use embedded_svc::mqtt::client::{Event, MessageImpl, Publish, QoS};
use std::include;

use esp_idf_svc::mqtt::client::*;
use esp_idf_sys::{self, EspError};

use embedded_svc::httpd::Result;

use esp_idf_svc::nvs::EspDefaultNvs;
use std::sync::{mpsc, Mutex};
use std::{sync::Arc, thread, time::Duration};

use crate::observer::Notifier;
use crate::thing_protocol;

type MqttClient<T> = (
    EspMqttClient<ConnState<MessageImpl, EspError>>,
    Arc<Mutex<Notifier<(Event<MessageImpl>, T)>>>,
);

pub fn mqtt_client<T>(user_data: T) -> Result<MqttClient<T>, EspError>
where
    T: std::marker::Send + 'static + Clone,
{
    println!("About to start MQTT client");
    let private_key = X509::pem(include_cstr!("../certificates/private.pem.key"));
    let root_ca = X509::pem(include_cstr!("../certificates/AmazonRootCA1.pem"));
    let cert = X509::pem(include_cstr!("../certificates/cert.crt"));

    let conf = MqttClientConfiguration {
        client_id: Some("esp32-2"),
        server_certificate: Some(root_ca),
        client_certificate: Some(cert),
        private_key: Some(private_key),

        ..Default::default()
    };

    let (client, mut connection) =
        EspMqttClient::new_with_conn(include_str!("../certificates/endpoint"), &conf)?;

    println!("MQTT client started");
    let notifier = Arc::new(Mutex::new(Notifier::new()));
    let shared_notifer = notifier.clone();
    thread::spawn(move || {
        println!("MQTT Listening for messages");
        while let Some(msg) = connection.next() {
            match msg {
                Err(e) => {
                    println!("MQTT Message ERROR: {}", e);
                }

                Ok(msg) => {
                    shared_notifer
                        .lock()
                        .unwrap()
                        .notify((msg, user_data.clone()));
                }
            }
        }

        println!("MQTT connection loop exit");
    });

    Ok((client, notifier))
}
