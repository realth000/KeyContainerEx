use guid_create::GUID;

// 61ab05a1-9464-41c3-8d74-3a563df8dd35
// 31c1f2e6-bf71-4350-be58-05216afc5aff
pub const CIPHER_AES_256: GUID = GUID::build_from_slice(&[
    0x31, 0xC1, 0xF2, 0xE6, 0xBF, 0x71, 0x43, 0x50, 0xBE, 0x58, 0x05, 0x21, 0x6A, 0xFC, 0x5A, 0xFF,
]);

pub enum CompressAlgorithm {
    None,
    Gzip,
    Count,
}
