use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem::size_of;

use alloc::vec::Vec;

use shim::io;
use shim::ioerr;
use shim::newioerr;
use shim::path;
use shim::path::Path;

use crate::mbr::MasterBootRecord;
use crate::traits::{BlockDevice, FileSystem};
use crate::util::SliceExt;
use crate::vfat::{BiosParameterBlock, CachedPartition, Partition};
use crate::vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Status};

/// A generic trait that handles a critical section as a closure
pub trait VFatHandle: Clone + Debug + Send + Sync {
    fn new(val: VFat<Self>) -> Self;
    fn lock<R>(&self, f: impl FnOnce(&mut VFat<Self>) -> R) -> R;
}

#[derive(Debug)]
pub struct VFat<HANDLE: VFatHandle> {
    phantom: PhantomData<HANDLE>,
    device: CachedPartition,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    rootdir_cluster: Cluster,
}

impl<HANDLE: VFatHandle> VFat<HANDLE> {
    pub fn from<T>(mut device: T) -> Result<HANDLE, Error>
    where
        T: BlockDevice + 'static,
    {
        // MasterBootRecord, BiosParameterBlock, CachedPartition
        
        // from BlockDevice produce a Result<HANDLE>
        // HANDLE is a VFatHandle -> use VFatHandle::new(VFat)
        // let mbr = MasterBootRecord::from(device)?;
        // let partition = Partition {
        //     start: u64::from(u32::from_be_bytes(mbr.partitions[0].relative_sector)),
        //     num_sectors: u64::from(u32::from_be_bytes(mbr.partitions[0].relative_sector)),
        //     sector_size: u64::from(u32::from_be_bytes(mbr.partitions[0].total_sectors))
        // };
        // let cached_partition = CachedPartition::new(device, partition);
        // let bpb = BiosParameterBlock::from(device, partition.start)?;
        // let reserved_secs = u64::from(u16::from_be_bytes(bpb.reserved_secs));
        // let secs_per_fat = u64::from(u32::from_be_bytes(bpb.secs_per_fat_2));
        // let num_fats = u64::from(bpb.fats);
        // let data_start = reserved_secs + secs_per_fat * num_fats;
        // let vfat = VFat::<HANDLE> {
        //     phantom: PhantomData,
        //     device: cached_partition,
        //     bytes_per_sector: u16::from_be_bytes(bpb.bytes_per_sec),
        //     sectors_per_cluster: bpb.sec_per_cluster,
        //     sectors_per_fat: u32::from_be_bytes(bpb.secs_per_fat_2),
        //     fat_start_sector: reserved_secs,
        //     data_start_sector: data_start,
        //     rootdir_cluster: Cluster::from(u32::from_be_bytes(bpb.cluster_root))
        // };
        // return Ok(VFatHandle::new(vfat));

        // locate the fat32 parititon (partition type = 0xB or 0xC)
        let mbr = MasterBootRecord::from(&mut device)?;
        let p = mbr.partitions.iter().find(|p| {
            p.partition_type == 0xB || p.partition_type == 0xC
        }).ok_or(Error::NotFound)?;

        let bpb = BiosParameterBlock::from(&mut device, p.relative_sector as u64)?;

        let partition = Partition {
            start: p.relative_sector as u64,
            // # virtual sectors * (size of physical sector / size of virtual sector) (?)
            num_sectors: (p.total_sectors as u64) * (device.sector_size() / (bpb.bytes_per_sec as u64)),
            sector_size: bpb.bytes_per_sec as u64 
        };

        let cached_partition = CachedPartition::new(device, partition);
        
        let vfat = VFat::<HANDLE> {
            phantom: PhantomData,
            device: cached_partition,
            bytes_per_sector: bpb.bytes_per_sec as u16,
            sectors_per_cluster: bpb.sec_per_cluster,
            sectors_per_fat: bpb.secs_per_fat_2 as u32,
            fat_start_sector: bpb.reserved_secs as u64,
            data_start_sector: (bpb.reserved_secs as u64) + (bpb.secs_per_fat_2 as u64) * (bpb.fats as u64),
            rootdir_cluster: Cluster::from(bpb.cluster_root as u32)
        };
        return Ok(VFatHandle::new(vfat));
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //
    //    fn read_cluster(
    //        &mut self,
    //        cluster: Cluster,
    //        offset: usize,
    //        buf: &mut [u8]
    //    ) -> io::Result<usize>;
    //
    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
    //    fn read_chain(
    //        &mut self,
    //        start: Cluster,
    //        buf: &mut Vec<u8>
    //    ) -> io::Result<usize>;
    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //
    //    fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry>;
}

impl<'a, HANDLE: VFatHandle> FileSystem for &'a HANDLE {
    type File = crate::traits::Dummy;
    type Dir = crate::traits::Dummy;
    type Entry = crate::traits::Dummy;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        unimplemented!("FileSystem::open()")
    }
}
