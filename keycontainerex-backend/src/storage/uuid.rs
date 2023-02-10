pub use guid_create::GUID;

#[derive(Debug)]
pub struct uuid {
    pub uuid_size: i8,
    pb_uuid: GUID,
}

impl uuid {
    pub fn new(create: bool) -> Self {
        if create {
            uuid::create_new()
        } else {
            uuid::zero()
        }
    }

    pub fn uuid_bytes(&self) -> &GUID {
        &self.pb_uuid
    }

    pub fn set_value(mut self, guid: GUID) {
        self.pb_uuid = guid;
    }

    fn create_new() -> Self {
        uuid {
            uuid_size: 0,
            pb_uuid: GUID::rand(),
        }
    }

    // TODO: Only init pb_uuid, do NOT set value?
    fn zero() -> Self {
        uuid::create_new()
    }
}
