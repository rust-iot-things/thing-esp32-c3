use embedded_svc::mqtt::client::{self, Event, MessageImpl, QoS};
use embedded_svc::utils::mqtt::client::ConnState;
use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_sys::EspError;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::app::Topics;
use crate::observer::Notifier;
use crate::{mqtt, thing_protocol};

const TOPICS: &[&str] = &["registry", "thing_input"];

pub(crate) struct ThingMQTT {
    mqtt: Arc<Mutex<EspMqttClient<ConnState<MessageImpl, EspError>>>>,
    notifier: Arc<Mutex<Notifier<(Event<MessageImpl>, Sender<Topics>)>>>,
}

impl ThingMQTT {
    pub fn new(tx: Sender<Topics>) -> Self {
        let (mqtt, notifier) = mqtt::mqtt_client(tx).unwrap();
        let mqtt = Arc::new(Mutex::new(mqtt));
        let mut thing_mqtt = ThingMQTT { mqtt, notifier };

        thing_mqtt.subscribe_to_topics();
        thing_mqtt.register_callbacks();
        thing_mqtt
    }

    fn subscribe_to_topics(&mut self) {
        for topic in TOPICS {
            self.mqtt
                .lock()
                .unwrap()
                .subscribe(topic, QoS::AtMostOnce)
                .unwrap();
        }
    }

    fn register_callbacks(&mut self) {
        self.notifier
            .lock()
            .unwrap()
            .register(thing_protocol::dispatch_event);
    }

    pub fn publish(&self, topic: &str, message: String) {
        println!("> {}: {}", topic, message);
        self.mqtt
            .lock()
            .unwrap()
            .publish(topic, client::QoS::AtMostOnce, false, message.as_bytes())
            .unwrap();
    }
}
