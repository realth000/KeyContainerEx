pub struct VariantDictionary {
    version: u16,
    critical: u16,
    info: u16,
}

impl Default for VariantDictionary {
    fn default() -> Self {
        VariantDictionary {
            version: 0x0100,
            critical: 0xFF00,
            info: 0x00FF,
        }
    }
}
