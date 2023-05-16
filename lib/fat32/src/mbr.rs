use core::fmt;
use shim::const_assert_size;
use shim::io;

use crate::traits::BlockDevice;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHS {
    // FIXME: Fill me in.
    bytes: [u8; 3]
}

impl CHS {
    fn head(&self) -> u8 {
        self.bytes[0]
    }

    fn sector(&self) -> u8 {
        self.bytes[1] & 0b0011_1111u8
    }
    
    fn cylinder(&self) -> u16 {
        (self.bytes[2] as u16) | ((self.bytes[1] & 0b1100_0000u8) as u16) << 2
    }
}

// FIXME: implement Debug for CHS
impl fmt::Debug for CHS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CHS")
            .field("cylinder", &format_args!("{:?}", self.cylinder()))
            .field("head", &format_args!("{:?}", self.head()))
            .field("sector", &format_args!("{:?}", self.sector()))
            .finish()
    }
}
// const_assert_size!(CHS, 3);

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct PartitionEntry {
    // FIXME: Fill me in.
    pub boot_indicator: u8,
    pub start: CHS,
    pub partition_type: u8,
    pub end: CHS,
    pub relative_sector: u32,
    pub total_sectors: u32
}

// FIXME: implement Debug for PartitionEntry
impl fmt::Debug for PartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PartitionEntry")
            .field("start", &format_args!("{:?}", self.start))
            .field("end", &format_args!("{:?}", self.end))
            .finish()
    }
}
// const_assert_size!(PartitionEntry, 16);

/// The master boot record (MBR).
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MasterBootRecord {
    // FIXME: Fill me in.
    pub bootstrap: [u8; 436],
    pub disk_id: [u8; 10],
    pub partitions: [PartitionEntry; 4],
    pub valid_bytes: [u8; 2]
}

// FIXME: implemente Debug for MaterBootRecord
impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterBootRecord")
            .field("disk_id", &format_args!("{:?}", self.disk_id))
            .field("partitions", &format_args!("{:?}", self.partitions))
            .finish()
    }
}
// const_assert_size!(MasterBootRecord, 512);

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partiion `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut buf = [0u8; 512];
        if let Err(e) = device.read_sector(0, &mut buf) {
            return Err(Error::Io(e));
        }

        // from_raw_parts method involves reconstructing the struct from the slice, i assume

        let pointer = &buf as *const [u8; 512] as *mut MasterBootRecord;
        let mbr = unsafe {(*(core::slice::from_raw_parts_mut(pointer, 1)))[0]};

        if mbr.valid_bytes != [0x55u8, 0xAAu8] {
            return Err(Error::BadSignature);
        }

        for i in 0..mbr.partitions.len() {
            if mbr.partitions[i].boot_indicator != 0 && mbr.partitions[i].boot_indicator != 0x80 {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }

        return Ok(mbr);

    }
}
