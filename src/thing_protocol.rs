use embedded_svc::mqtt::client::{Event, Message, MessageImpl};

use protocol::message_set_name::SetNameDescirption;
use protocol::parser_registry::{parse, RegistryType};

use std::str::from_utf8;
use std::sync::{Arc, Mutex};

use crate::thing;

pub(crate) fn dispatch_event(event: &(Event<MessageImpl>, Arc<Mutex<thing::Thing>>)) {
    if let Event::Received(message) = event.0.clone() {
        dispatch_message(message, event.1.clone())
    }
}

fn dispatch_message(message: MessageImpl, thing: Arc<Mutex<thing::Thing>>) {
    let payload: &str = from_utf8(message.data()).unwrap();
    match message.topic().unwrap() {
        "registry" => handle_registry(payload, thing),
        "thing_input" => handle_thing_input(payload, thing),
        _ => println!("unknwon topic"),
    }
}

fn handle_thing_input(payload: &str, _thing: Arc<Mutex<thing::Thing>>) {
    println!("thing_input: {}", payload);
}

fn handle_registry(payload: &str, thing: Arc<Mutex<thing::Thing>>) {
    println!("registry: {}", payload);

    match parse(payload) {
        RegistryType::SetNameType(it) => set_thing_name(it, thing),
        RegistryType::None => {}
        _ => {}
    }
}

fn set_thing_name(set_name_description: SetNameDescirption, thing: Arc<Mutex<thing::Thing>>) {
    if set_name_description.set_name.id == thing.lock().unwrap().get_id() {
        thing
            .lock()
            .unwrap()
            .set_name(set_name_description.set_name.name);
    }
}
