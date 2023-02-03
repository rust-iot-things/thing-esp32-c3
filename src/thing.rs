use esp_idf_sys::EspError;
use rand::Rng;

use crate::{lamp::RGB, nvs_uuid::get_uuid};

pub struct Thing<'a> {
    id: u128,
    name: String,
    rgb: RGB<'a>,
}

impl Thing<'_> {
    pub fn new() -> Result<Self, EspError> {
        // TODO: Error handling for RGB
        let rgb = RGB::new();

        let uuid = match get_uuid() {
            Ok(uuid) => uuid,
            Err(e) => return Err(e),
        };

        Ok(Self {
            id: uuid,
            name: "".to_string(),
            rgb,
        })
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }

    pub(crate) fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn is_registered(&self) -> bool {
        self.name != "".to_string()
    }

    pub(crate) fn get_temperature(&self) -> f32 {
        19.0 + rand::thread_rng().gen_range(0.0..2.0)
    }

    pub(crate) fn get_humidity(&self) -> u8 {
        45 + rand::thread_rng().gen_range(0..10)
    }

    pub(crate) fn set_lamp_state(&mut self, state: bool) {
        if state {
            self.rgb.on();
        } else {
            self.rgb.off();
        }
    }

    pub(crate) fn set_lamp_rgb(&mut self, r: u32, g: u32, b: u32) {
        self.rgb.set(r, g, b);
    }
}
