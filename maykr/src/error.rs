#![allow(unused)]
#![allow(clippy::enum_variant_names)]

use minhook::MH_STATUS;

#[repr(u32)]
#[derive(Debug)]
pub enum Error {
    MinHookError(MH_STATUS) = 0xBEEF,
    PatternNotFound = 0xDEAD,
    PathError = 0xC0DE,
    IoError = 0xD00D,
    UnhandledError = 0xCAFE,
}

impl Error {
    pub fn discriminant(&self) -> u32 {
        unsafe { *<*const _>::from(self).cast::<u32>() }
    }
}

impl From<MH_STATUS> for Error {
    fn from(value: MH_STATUS) -> Self {
        Self::MinHookError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}
