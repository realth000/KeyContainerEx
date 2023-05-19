use std::error::Error;
use std::fs::File;
use std::path::PathBuf;

use keepass::{Database, DatabaseKey};
use keepass::config::{CompressionConfig, DatabaseConfig, DatabaseVersion, InnerCipherConfig, OuterCipherConfig};
use keepass::config::KdfConfig::Aes;

const DEFAULT_DB_CONFIG: DatabaseConfig = DatabaseConfig {
    version: DatabaseVersion::KDB4(0),
    outer_cipher_config: OuterCipherConfig::AES256,
    compression_config: CompressionConfig::GZip,
    inner_cipher_config: InnerCipherConfig::ChaCha20,
    kdf_config: Aes { rounds: 16 },
};

pub fn init_storage(path: PathBuf, password: &str) -> Result<(), Box<dyn Error>> {
    let db = Database::new(DEFAULT_DB_CONFIG);
    Ok(())
}

pub fn open_storage(path: PathBuf, password: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
