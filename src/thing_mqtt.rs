use embedded_svc::mqtt::client::{self, Event, MessageImpl, QoS};
use embedded_svc::utils::mqtt::client::ConnState;
use esp_idf_svc::mqtt::client::EspMqttClient;
use esp_idf_sys::EspError;
use protocol::{
    message_measurement_humidity, message_measurement_temperature, message_request_registartion,
};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{mqtt, observer, thing, thing_protocol};

const TOPICS: &[&str] = &["registry", "thing_input"];

type ArcNotifier = Arc<Mutex<observer::Notifier<(Event<MessageImpl>, Arc<Mutex<thing::Thing>>)>>>;

pub(crate) struct ThingMQTT {
    mqtt: Arc<Mutex<EspMqttClient<ConnState<MessageImpl, EspError>>>>,
    notifier: ArcNotifier,
    thing: Arc<Mutex<thing::Thing>>,
}

impl ThingMQTT {
    pub fn new() -> Self {
        let thing = Arc::new(Mutex::new(thing::Thing::new()));
        let (mqtt, notifier) = mqtt::mqtt_client(thing.clone()).unwrap();
        let mqtt = Arc::new(Mutex::new(mqtt));
        let mut thing_mqtt = ThingMQTT {
            mqtt,
            notifier,
            thing,
        };

        thing_mqtt.subscribe_to_topics();
        thing_mqtt.register_callbacks();
        thing_mqtt.register_device();
        thing_mqtt.read_temperature();
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

    fn register_device(&mut self) {
        let message = message_request_registartion::create(self.thing.lock().unwrap().get_id());
        self.publish("registry", message);
    }

    fn publish(&self, topic: &str, message: String) {
        println!("> {}: {}", topic, message);
        self.mqtt
            .lock()
            .unwrap()
            .publish(topic, client::QoS::AtMostOnce, false, message.as_bytes())
            .unwrap();
    }

    fn read_temperature(&mut self) {
        let thing = self.thing.clone();
        let mqtt = self.mqtt.clone();
        thread::spawn(move || loop {
            if thing.lock().unwrap().is_registered() {
                let id = thing.lock().unwrap().get_id();
                let humidity = thing.lock().unwrap().get_humidity();
                let humidity_message = message_measurement_humidity::create(id, humidity);
                let temperature = thing.lock().unwrap().get_temperature();
                let temperature_message = message_measurement_temperature::create(id, temperature);

                mqtt.lock()
                    .unwrap()
                    .publish(
                        "thing_input",
                        client::QoS::AtMostOnce,
                        false,
                        humidity_message.as_bytes(),
                    )
                    .unwrap();
                mqtt.lock()
                    .unwrap()
                    .publish(
                        "thing_input",
                        client::QoS::AtMostOnce,
                        false,
                        temperature_message.as_bytes(),
                    )
                    .unwrap();
            }
            thread::sleep(Duration::from_secs(15));
        });
    }
}
