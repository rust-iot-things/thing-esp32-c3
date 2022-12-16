use rand::Rng;

#[derive(Clone)]
pub(crate) struct Thing {
    id: u64,
    name: String,
}

impl Thing {
    pub fn new() -> Self {
        Self {
            id: 1771,
            name: "".to_string(),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub(crate) fn set_name(&mut self, name: String) {
        println!("set name: {}", name);
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
}
