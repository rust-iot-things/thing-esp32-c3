use byteorder::{BigEndian, ReadBytesExt};
use embedded_svc::storage::RawStorage;
use esp_idf_svc::nvs::{self, EspNvsPartition, NvsDefault};
use esp_idf_sys::*;
use uuid::Uuid;

// TODO: Use Uuid v7 with timestamp
pub fn get_uuid(partition: &EspNvsPartition<NvsDefault>) -> Result<u128, EspError> {
    let nvs = nvs::EspDefaultNvs::new(partition.to_owned(), "uuidstorage", true).unwrap();
    let cointains = nvs.contains("uuid").unwrap();
    match cointains {
        true => read_uuid(nvs),
        false => create_uuid(nvs),
    }
}

fn read_uuid(nvs: nvs::EspNvs<nvs::NvsDefault>) -> Result<u128, EspError> {
    let mut uuid = [0; 16];
    match nvs.get_raw("uuid", &mut uuid) {
        Ok(result) => {
            let uuid = result.unwrap().read_u128::<BigEndian>().unwrap();
            println!("read uuid: {:?}", uuid);
            return Ok(uuid);
        }
        Err(e) => {
            println!("failed to read uuid");
            return Err(e);
        }
    }
}

fn create_uuid(mut nvs: nvs::EspNvs<nvs::NvsDefault>) -> Result<u128, EspError> {
    let uuid = Uuid::new_v4();
    let uuid: &[u8; 16] = uuid.as_bytes();
    match nvs.set_raw("uuid", uuid) {
        Ok(_) => {
            println!("set uuid: {:?}", uuid);
            return Ok(u128::from_be_bytes(*uuid));
        }
        Err(e) => {
            println!("failed to set uuid");
            return Err(e);
        }
    }
}
