use std::{fmt, mem, slice};
use std::ops::{Deref, DerefMut};

use benfs_uuid::BenFS_Uuid;

use {X_BLOCK_SIZE, BEN_SIGNATURE, BEN_VERSION};

/// BenHeader is the `header` for the benFS local file system
#[repr(packed)]
pub struct BenHeader {
    pub benfs_signature: [u8; 8],
    pub benfs_version: u64,
    pub benfs_uuid: [u8; 16],
    pub benfs_size: u64,
    pub benfs_root: u64,
    pub benfs_free: u64,
    pub benfs_padding: [u8; X_BLOCK_SIZE as usize - 56]
}

impl BenHeader {
    pub fn default() -> BenHeader {
        BenHeader {
            benfs_signature: [0; 8],
            benfs_version: 0,
            benfs_uuid: [0; 16],
            benfs_size: 0,
            benfs_root: 0,
            benfs_free: 0,
            benfs_padding: [0; X_BLOCK_SIZE as usize - 56]
        }
    }

    pub fn new(benfs_size: u64, benfs_root: u64, benfs_free: u64) -> BenHeader {
        let benfs_uuid = BenFS_Uuid::new_v4();
        BenHeader {
            benfs_signature: *BEN_SIGNATURE,
            benfs_version: BEN_VERSION,
            benfs_uuid: *benfs_uuid.as_bytes(),
            benfs_size: benfs_size,
            benfs_root: benfs_root,
            benfs_free: benfs_free,
            benfs_padding: [0; X_BLOCK_SIZE as usize - 56]
        }
    }

    pub fn valid(&self) -> bool {
        &self.benfs_signature == BENFS_SIGNATURE && self.benfs_version == BENFS_VERSION
    }
}

impl fmt::Debug for BenHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            f.debug_struct("BenHeader")
                .field("benfs_signature", &self.benfs_signature)
                .field("benfs_version", &self.benfs_version)
                .field("benfs_uuid", &self.benfs_uuid)
                .field("benfs_size", &self.benfs_size)
                .field("benfs_root", &self.benfs_root)
                .field("benfs_free", &self.benfs_free)
                .finish()
        }
    }
}

impl Deref for BenHeader {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const BenHeader as *const u8, mem::size_of::<BenHeader>()) as &[u8]
        }
    }
}

impl DerefMut for BenHeader {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut BenHeader as *mut u8, mem::size_of::<BenHeader>()) as &mut [u8]
        }
    }
}

#[test]
fn benheader_size_test() {
    assert_eq!(mem::size_of::<BenHeader>(), X_BLOCK_SIZE as usize);
}
