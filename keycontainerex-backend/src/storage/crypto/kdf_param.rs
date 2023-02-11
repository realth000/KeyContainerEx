use crate::storage::uuid::Uuid;

pub struct KdfParam {
    param_uuid: String,
    uuid: Uuid,
}

impl KdfParam {
    pub fn new(uuid: Uuid) -> Self {
        KdfParam {
            param_uuid: uuid.to_string(),
            uuid,
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}
