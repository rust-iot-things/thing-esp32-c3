use embedded_svc::mqtt::client::{Event, Message, MessageImpl};

use std::str::from_utf8;
use std::sync::mpsc::Sender;

use crate::app::Topics;

pub(crate) fn dispatch_event(event: &(Event<MessageImpl>, Sender<Topics>)) {
    if let Event::Received(message) = event.0.clone() {
        dispatch_message(message, event.1.clone())
    }
}

fn dispatch_message(message: MessageImpl, tx: Sender<Topics>) {
    let payload: &str = from_utf8(message.data()).unwrap();
    match message.topic().unwrap() {
        "registry" => handle_registry(payload, tx),
        "thing_input" => handle_thing_input(payload, tx),
        _ => println!("unknwon topic"),
    }
}

fn handle_thing_input(payload: &str, tx: Sender<Topics>) {
    let it = protocol::parser_thing_input::parse(payload);
    tx.send(Topics::ThingInput(it)).unwrap();
}

fn handle_registry(payload: &str, tx: Sender<Topics>) {
    let it = protocol::parser_registry::parse(payload);
    tx.send(Topics::Registry(it)).unwrap();
}
