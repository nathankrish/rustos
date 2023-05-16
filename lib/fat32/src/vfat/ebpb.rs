use core::fmt;
use shim::const_assert_size;

use crate::traits::BlockDevice;
use crate::vfat::Error;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BiosParameterBlock {
    // FIXME: Fill me in.
    pub first_three: [u8; 3],
    pub oem_id: u64,
    pub bytes_per_sec: u16,
    pub sec_per_cluster: u8,
    pub reserved_secs: u16,
    pub fats: u8,
    pub max_dirs: u16,
    pub logical_sectors_1: u16,
    pub media_type: u8,
    pub secs_per_fat_1: u16,
    pub secs_per_track: u16,
    pub heads: u16,
    pub hidden_secs: u32,
    pub logical_sectors_2: u32,
    pub secs_per_fat_2: u32,
    pub flags: u16,
    pub fat_ver: u16,
    pub cluster_root: u32,
    pub fsinfo_sec_no: u16,
    pub backup_sec_no: u16,
    pub reserved: [u8; 12],
    pub drive_no: u8,
    pub flags_windows_nt: u8,
    pub signature: u8,
    pub volume_id_serial: u32,
    pub volume_label_str: [u8; 11],
    pub system_id: u64,
    pub boot_code: [u8; 420],
    pub boot_partition_signature: u16
}

// const_assert_size!(BiosParameterBlock, 512);

impl BiosParameterBlock {
    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        let mut buf = [0u8; 512];
        if let Err(e) = device.read_sector(sector, &mut buf) {
            return Err(Error::Io(e));
        }

        let pointer = &buf as *const [u8; 512] as *mut BiosParameterBlock;
        let ebpb = unsafe {(*(core::slice::from_raw_parts_mut(pointer, 1)))[0]};
        if ebpb.boot_partition_signature != 0x55AAu16 {
            return Err(Error::BadSignature);
        }
        return Ok(ebpb);
    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BiosParameterBlock")
            .field("reserved_sectors", &format_args!("{:?}", self.reserved_secs))
            .finish()
    }
}
