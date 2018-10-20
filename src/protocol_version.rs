use std::hash::Hasher;
use std::sync::Mutex;

use crc::Hasher32;
use crc::crc32;

pub use net::constants::PROTOCOL_VERSION;

lazy_static! {
    // The CRC32 of the current protocol version.
    static ref VERSION_CRC32: Mutex<u32> = Mutex::new(crc32::checksum_ieee(PROTOCOL_VERSION.as_bytes()));
}

/// Wrapper to provide some functions to perform with the current protocol version.
pub struct ProtocolVersion;

impl ProtocolVersion
{
    /// Get the current protocol version.
    pub fn get_version() -> &'static str
    {
        return PROTOCOL_VERSION
    }

    /// This will return the crc32 from the current protocol version.
    pub fn get_crc32() -> u32 {
        VERSION_CRC32.lock().unwrap().clone()
    }

    /// Validate a crc32 with the current protocol version and return the results.
    pub fn valid_version(protocol_version_crc32: u32) -> bool {
        protocol_version_crc32 == VERSION_CRC32.lock().unwrap().clone()
    }
}