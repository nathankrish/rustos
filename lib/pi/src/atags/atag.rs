use crate::atags::raw;

pub use crate::atags::raw::{Core, Mem};

/// An ATAG.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        return match self {
            Atag::Core(x) => Some(x),
            _ => None
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        return match self {
            Atag::Mem(x) => Some(x),
            _ => None
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        return match self {
            Atag::Cmd(x) => Some(x),
            _ => None
        }
    }
}

// FIXME: Implement `From<&raw::Atag> for `Atag`.
impl From<&'static raw::Atag> for Atag {
    fn from(atag: &'static raw::Atag) -> Atag {
        // FIXME: Complete the implementation below.
        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => {
                    Atag::Core(core)
                },
                (raw::Atag::MEM, &raw::Kind { mem }) => {
                    Atag::Mem(mem)
                },
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => {
                    let mut len = 0;
                    let ptr = cmd as *const raw::Cmd as usize;
                    while *((ptr + len * 8) as *const char) != '\0' {
                        len += 1;
                    }
                    // account for '\0'
                    len += 1;
                    let slice = core::slice::from_raw_parts(cmd as *const raw::Cmd as *const _, len);
                    let string = core::str::from_utf8(slice).expect("ptr->str");
                    Atag::Cmd(string)
                },
                (raw::Atag::NONE, _) => {
                    Atag::None
                },
                (id, _) => {
                   Atag::Unknown(id)
                },
            }
        }
    }
}
