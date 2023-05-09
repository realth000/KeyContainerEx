use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use keepass::config::DatabaseConfig;
use keepass::{Database, DatabaseKey};

pub fn init_storage(path: PathBuf, password: &str) -> Result<(), Box<dyn Error>> {
    Database::new(DatabaseConfig {
        version: (),
        outer_cipher_config: OuterCipherConfig::AES256,
        compression_config: CompressionConfig::None,
        inner_cipher_config: InnerCipherConfig::Plain,
        kdf_config: (),
    });
    let db = Database::open(&mut File::open(path)?, DatabaseKey::with_password(password));
    Ok(())
}

pub fn open_storage(path: PathBuf, password: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
