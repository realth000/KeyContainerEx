pub enum KdbxFormat {
    Default,
    PlainXml,
}

const FILE_SIGNATURE1: u32 = 0x9AA2D903;

const FILE_SIGNATURE2: u32 = 0xB54BFB67;

const FILE_VERSION_32: u32 = 0x00040000;

const FILE_VERSION_32_4: u32 = 0x00040000;

const FILE_VERSION_32_3: u32 = 0x00030001;

const FILE_SIGNATURE_OLD_1: u32 = 0x9AA2D903;

const FILE_SIGNATURE_OLD_2: u32 = 0xB54BFB65;

const FILE_SIGNATURE_PRE_RELEASE_1: u32 = 0x9AA2D903;

const FILE_SIGNATURE_PRE_RELEASE_2: u32 = 0xB54BFB66;

pub struct KdbxFile {
    file_type: KdbxFormat,
    database: Databa,
}
