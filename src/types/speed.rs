use std::fmt;

/// Represents USBFS transfer speeds.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UsbfsSpeed {
    #[default]
    Unknown = 0,
    Low = 1,
    Full = 2,
    High = 3,
    Wireless = 4,
    Super = 5,
    SuperPlus = 6,
}

impl UsbfsSpeed {
    /// Creates a new [UsbfsSpeed].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Gets the inner representation of the [UsbfsSpeed].
    pub const fn inner(&self) -> u32 {
        *self as u32
    }

    /// Converts into the inner representation of the [UsbfsSpeed].
    pub fn into_inner(self) -> u32 {
        self as u32
    }
}

impl From<&UsbfsSpeed> for &'static str {
    fn from(val: &UsbfsSpeed) -> Self {
        match val {
            UsbfsSpeed::Unknown => "unknown",
            UsbfsSpeed::Low => "low",
            UsbfsSpeed::Full => "full",
            UsbfsSpeed::High => "high",
            UsbfsSpeed::Wireless => "wireless",
            UsbfsSpeed::Super => "super",
            UsbfsSpeed::SuperPlus => "super plus",
        }
    }
}

impl From<UsbfsSpeed> for &'static str {
    fn from(val: UsbfsSpeed) -> Self {
        (&val).into()
    }
}

impl fmt::Display for UsbfsSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
