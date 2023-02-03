use std::sync::mpsc::channel;
use std::time::Duration;
use std::{sync::mpsc::Receiver, thread};

use esp_idf_svc::nvs::{EspNvsPartition, NvsDefault};
use esp_idf_sys::EspError;
use protocol::message_set_name::SetNameDescirption;
use protocol::parser_registry::RegistryType;
use protocol::parser_thing_input::ThingInputType;
use protocol::{
    message_measurement_humidity, message_measurement_temperature, message_request_registartion,
};

use crate::thing::Thing;
use crate::thing_mqtt::ThingMQTT;

pub enum Topics {
    Registry(RegistryType),
    ThingInput(ThingInputType),
    Timer(Duration),
}

pub fn start(nvs: &EspNvsPartition<NvsDefault>) -> Result<(), EspError> {
    let (tx, rx) = channel::<Topics>();
    let mqtt = ThingMQTT::new(tx.clone());

    match Thing::new(nvs) {
        Ok(mut thing) => {
            register_device(&mut thing, &mqtt);
            thing.set_lamp_rgb(100, 100, 100);
            thing.set_lamp_state(true);

            // create Timer event every 10 seconds
            thread::spawn(move || loop {
                let duration = Duration::from_secs(10);
                thread::sleep(duration);
                tx.send(Topics::Timer(duration)).unwrap();
            });

            loop {
                match next_event(&rx) {
                    Some(event) => handle_event(event, &mut thing, &mqtt),
                    None => thread::sleep(Duration::from_millis(100)),
                }
            }
        }
        Err(error) => return Err(error),
    }
}

fn register_device(thing: &mut Thing, mqtt: &ThingMQTT) {
    let message = message_request_registartion::create(thing.get_id());
    mqtt.publish("registry", message);
}

fn next_event(rx: &Receiver<Topics>) -> Option<Topics> {
    match rx.recv_timeout(Duration::from_millis(10)) {
        Ok(event) => Option::Some(event),
        Err(_) => Option::None,
    }
}

fn handle_event(event: Topics, thing: &mut Thing, mqtt: &ThingMQTT) {
    match event {
        Topics::Registry(it) => handle_registry(it, thing),
        Topics::ThingInput(it) => handle_thing_input(it, thing),
        Topics::Timer(_) => handle_timer(thing, mqtt),
    }
}

fn handle_timer(thing: &mut Thing, mqtt: &ThingMQTT) {
    if thing.is_registered() {
        let message =
            message_measurement_temperature::create(thing.get_id(), thing.get_temperature());
        mqtt.publish("thing_input", message);
        let message = message_measurement_humidity::create(thing.get_id(), thing.get_humidity());
        mqtt.publish("thing_input", message);
    }
}

fn handle_registry(registry: RegistryType, thing: &mut Thing) {
    match registry {
        RegistryType::SetNameType(set_name_description) => {
            set_thing_name(thing, set_name_description)
        }
        _ => {}
    }
}

fn set_thing_name(thing: &mut Thing, set_name_description: SetNameDescirption) {
    if set_name_description.set_name.id.eq(&thing.get_id()) {
        thing.set_name(set_name_description.set_name.name);
    } else {
    }
}

fn handle_thing_input(thing_input: ThingInputType, thing: &mut Thing) {
    match thing_input {
        ThingInputType::LampRGB(it) => {
            set_lamp_rgb(thing, it);
        }
        ThingInputType::LampState(it) => {
            set_lamp_state(thing, it);
        }
        _ => {}
    }
}

fn set_lamp_rgb(
    thing: &mut Thing,
    lamp_rgb_description: protocol::message_lamp_rgb::LampRGBDescirption,
) {
    if lamp_rgb_description.lamp_rgb.id.eq(&thing.get_id()) {
        thing.set_lamp_rgb(
            lamp_rgb_description.lamp_rgb.r,
            lamp_rgb_description.lamp_rgb.g,
            lamp_rgb_description.lamp_rgb.b,
        );
    }
}

fn set_lamp_state(
    thing: &mut Thing,
    lamp_state_description: protocol::message_lamp_state::LampStateDescirption,
) {
    if lamp_state_description.lamp_state.id.eq(&thing.get_id()) {
        thing.set_lamp_state(lamp_state_description.lamp_state.state);
    }
}
