// use std::fmt::{self, Debug};
// use std::io;
// use std::io::prelude::*;
// use std::io::Cursor;
// use std::path::Path;
// use std::sync::{Arc, Mutex};

// use crate::mbr;
// use crate::traits::*;
// use crate::vfat;

// use mbr::{MasterBootRecord, PartitionEntry, CHS};
// use vfat::{BiosParameterBlock, VFat, VFatHandle};

// #[cfg(test)]
// use super::*;

// #[test]
// fn test_mock_mbr() {
//     let mut data = [0u8; 512];
    
//     // valid signature
//     data[510..512].copy_from_slice(&[0x55,0xAA]);
//     // partition entry boot indicators
//     data[446..447].copy_from_slice(&[0x0]);
//     data[462..463].copy_from_slice(&[0x80]);
//     data[478..479].copy_from_slice(&[0x0]);
//     data[494..495].copy_from_slice(&[0x80]);

//     for i in 0..446 {
//         data[i] = i as u8;
//     }

//     let mbr = MasterBootRecord::from(Cursor::new(&mut data[..])).unwrap();
//     let mut expected_bootstrap = [0u8; 436];
//     for i in 0..436 {
//         expected_bootstrap[i] = i as u8;
//     }
// }

// fn test_mock_ebpb() {

// }
