use std::fmt;

/// Convenience alias for the library `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the USBFS crate.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Ioctl(String),
}

impl From<nix::errno::Errno> for Error {
    fn from(err: nix::errno::Errno) -> Self {
        Self::Ioctl(format!("{err}"))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ioctl(err) => write!(f, "IOCTL error: {err}"),
        }
    }
}
