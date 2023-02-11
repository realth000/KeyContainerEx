use crate::storage::{
    define::{CompressAlgorithm, CIPHER_AES_256},
    deleted_object::DeletedObject,
    group::Group,
};
use guid_create::GUID;

const DEFAULT_HISTORY_MAX_ITEMS: i32 = 10;
const DEFAULT_HISTORY_MAX_SIZE: i32 = 6 * 1024 * 1024;
static mut PRIMARY_CREATED: bool = false;

#[derive(Debug)]
pub struct Database {
    pub root_group: Group,
    deleted_objects: Vec<DeletedObject>,
    pub uuid_data_cipher: GUID,
    pub ca_compression: CompressAlgorithm,
}

impl Database {
    pub fn deleted_objects(&self) -> &Vec<DeletedObject> {
        &self.deleted_objects
    }
}
