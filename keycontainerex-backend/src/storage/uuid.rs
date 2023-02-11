pub use guid_create::GUID;

#[derive(Debug)]
pub struct Uuid {
    pub uuid_size: i8,
    pb_uuid: GUID,
}

impl Uuid {
    pub fn new(create: bool) -> Self {
        if create {
            Uuid::create_new()
        } else {
            Uuid::zero()
        }
    }

    pub fn uuid_bytes(&self) -> &GUID {
        &self.pb_uuid
    }

    pub fn set_value(mut self, guid: GUID) {
        self.pb_uuid = guid;
    }

    pub fn to_string(&self) -> String {
        self.pb_uuid.to_string()
    }

    fn create_new() -> Self {
        Uuid {
            uuid_size: 0,
            pb_uuid: GUID::rand(),
        }
    }

    // TODO: Only init pb_uuid, do NOT set value?
    fn zero() -> Self {
        Uuid::create_new()
    }
}
