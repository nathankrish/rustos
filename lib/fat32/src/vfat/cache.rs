use alloc::boxed::Box;
use alloc::vec::Vec;
use core::fmt;
use hashbrown::HashMap;
use shim::io;

use crate::traits::BlockDevice;
use std::ops::DerefMut;

#[derive(Debug)]
struct CacheEntry {
    data: Vec<u8>,
    dirty: bool,
}

pub struct Partition {
    /// The physical sector where the partition begins.
    pub start: u64,
    /// Number of sectors
    pub num_sectors: u64,
    /// The size, in bytes, of a logical sector in the partition.
    pub sector_size: u64,
}

pub struct CachedPartition {
    device: Box<dyn BlockDevice>,
    cache: HashMap<u64, CacheEntry>,
    partition: Partition,
    buffer: Vec<u8>
}

impl CachedPartition {
    /// Creates a new `CachedPartition` that transparently caches sectors from
    /// `device` and maps physical sectors to logical sectors inside of
    /// `partition`. All reads and writes from `CacheDevice` are performed on
    /// in-memory caches.
    ///
    /// The `partition` parameter determines the size of a logical sector and
    /// where logical sectors begin. An access to a sector `0` will be
    /// translated to physical sector `partition.start`. Virtual sectors of
    /// sector number `[0, num_sectors)` are accessible.
    ///
    /// `partition.sector_size` must be an integer multiple of
    /// `device.sector_size()`.
    ///
    /// # Panics
    ///
    /// Panics if the partition's sector size is < the device's sector size.
    pub fn new<T>(device: T, partition: Partition) -> CachedPartition
    where
        T: BlockDevice + 'static,
    {
        assert!(partition.sector_size >= device.sector_size());

        CachedPartition {
            device: Box::new(device),
            cache: HashMap::new(),
            partition: partition,
            buffer: Vec::new()
        }
    }

    /// Returns the number of physical sectors that corresponds to
    /// one logical sector.
    fn factor(&self) -> u64 {
        self.partition.sector_size / self.device.sector_size()
    }

    /// Maps a user's request for a sector `virt` to the physical sector.
    /// Returns `None` if the virtual sector number is out of range.
    fn virtual_to_physical(&self, virt: u64) -> Option<u64> {
        if virt >= self.partition.num_sectors {
            return None;
        }

        let physical_offset = virt * self.factor();
        let physical_sector = self.partition.start + physical_offset;

        Some(physical_sector)
    }

    fn load_sector(&mut self, phys_sector: u64, dirty: bool) -> io::Result<()>{
        // check if sector in cache
        if !self.cache.contains_key(&phys_sector) {
            // clear the buffer
            self.buffer.clear();
            self.buffer.reserve(self.partition.sector_size as usize);
            // write the bytes from the device into the buffer
            let slice = self.buffer.as_mut_slice();
            let device = self.device.deref_mut();
            device.read_sector(phys_sector, slice)?;
            // copy the bytes from buffer into vec to insert into cache
            let mut data_vec = Vec::new();
            data_vec.copy_from_slice(slice);
            self.cache.insert(phys_sector, CacheEntry {data: data_vec, dirty: dirty});
        } else if dirty {
            // mark the cache 
            let entry = self.cache.get_mut(&phys_sector).unwrap();
            entry.dirty = true;
        }
        return Ok(());
    }

    /// Returns a mutable reference to the cached sector `sector`. If the sector
    /// is not already cached, the sector is first read from the disk.
    ///
    /// The sector is marked dirty as a result of calling this method as it is
    /// presumed that the sector will be written to. If this is not intended,
    /// use `get()` instead.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error reading the sector from the disk.
    pub fn get_mut(&mut self, sector: u64) -> io::Result<&mut [u8]> {
        
        /*
            compute the virtual sector no.
            if the sector is not in the cache
                read from block device into buffer
                return IO error if necessary
                create CacheEntry marked dirty
                put CacheEntry in cache
                return data as slice
            else
                fetch CacheEntry from cache
                return data as slice
         */
        let phys = self.virtual_to_physical(sector).ok_or(io::ErrorKind::InvalidInput)?;
        self.load_sector(phys, true)?;
        // need to use self.buffer because its lifetime is greater than this function
        // cannot create a Vec here and return slice because lifetime will disappear 
        let cache_entry = self.cache.get(&phys).ok_or(io::ErrorKind::NotFound)?;
        self.buffer.clear();
        self.buffer.reserve(self.partition.sector_size as usize);
        self.buffer.copy_from_slice(cache_entry.data.as_slice());
        return Ok(self.buffer.as_mut_slice());        
    }

    /// Returns a reference to the cached sector `sector`. If the sector is not
    /// already cached, the sector is first read from the disk.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error reading the sector from the disk.
    pub fn get(&mut self, sector: u64) -> io::Result<&[u8]> {
        let phys = self.virtual_to_physical(sector).ok_or(io::ErrorKind::InvalidInput)?;
        self.load_sector(phys, false)?;
        let cache_entry = self.cache.get(&phys).ok_or(io::ErrorKind::NotFound)?;
        self.buffer.clear();
        self.buffer.reserve(self.partition.sector_size as usize);
        self.buffer.copy_from_slice(cache_entry.data.as_slice());
        return Ok(self.buffer.as_slice());
    }
}

// FIXME: Implement `BlockDevice` for `CacheDevice`. The `read_sector` and
// `write_sector` methods should only read/write from/to cached sectors.
impl BlockDevice for CachedPartition {
    fn sector_size(&self) -> u64 {
        self.partition.sector_size
    }

    fn read_sector(&mut self, sector: u64, buf: &mut [u8]) -> io::Result<usize> {
        let slice = self.get(sector)?;
        buf.copy_from_slice(slice);
        return Ok(self.sector_size() as usize);
    }

    fn write_sector(&mut self, sector: u64, buf: &[u8]) -> io::Result<usize> {
        let mut_slice = self.get_mut(sector)?;
        mut_slice.copy_from_slice(buf);
        return Ok(self.sector_size() as usize);
    }
}

impl fmt::Debug for CachedPartition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CachedPartition")
            .field("device", &"<block device>")
            .field("cache", &self.cache)
            .finish()
    }
}
