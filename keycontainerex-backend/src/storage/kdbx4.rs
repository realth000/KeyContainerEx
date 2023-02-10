pub enum KdbxFormat {
    Default,
    PlainXml,
}

const file_signature1: u32 = 0x9AA2D903;

const file_signature2: u32 = 0xB54BFB67;

const file_version32: u32 = 0x00040000;

const file_version32_4: u32 = 0x00040000;

const file_version32_3: u32 = 0x00030001;

const file_signature_old1: u32 = 0x9AA2D903;

const file_signature_old2: u32 = 0xB54BFB65;

const file_signature_pre_release1: u32 = 0x9AA2D903;

const file_signature_pre_release2: u32 = 0xB54BFB66;

pub struct KdbxFile {
    file_type: KdbxFormat,
}
