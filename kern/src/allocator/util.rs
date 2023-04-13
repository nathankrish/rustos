/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align & (align - 1) != 0 {
        panic!("Align is not a power of two");
    }
    let remainder = addr % align;
    return addr - remainder;
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2
/// or aligning up overflows the address.
pub fn align_up(addr: usize, align: usize) -> usize {
    if align & (align - 1) != 0 {
        panic!("Align is not a power of two");
    }
    let remainder = addr % align;
    if remainder == 0 {
        return addr;
    }
    let result = addr + (align - remainder);
    if result < addr {
        panic!("Overflowed the address");
    }
    return result;
}
