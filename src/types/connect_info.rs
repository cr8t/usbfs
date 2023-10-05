use std::fmt;

/// Represents USBFS connection information.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UsbfsConnectInfo {
    devnum: u32,
    slow: u8,
}

impl UsbfsConnectInfo {
    /// Creates a new [UsbfsConnectInfo].
    pub const fn new() -> Self {
        Self {
            devnum: 0,
            slow: 0,
        }
    }

    /// Gets the device number.
    pub const fn devnum(&self) -> u32 {
        self.devnum
    }

    /// Gets the slow field.
    pub const fn slow(&self) -> u8 {
        self.slow
    }
}

impl fmt::Display for UsbfsConnectInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""devnum": {}, "#, self.devnum)?;
        write!(f, r#""slow": {}"#, self.slow)?;
        write!(f, "}}")
    }
}
