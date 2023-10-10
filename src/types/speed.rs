use std::fmt;

pub const SPEED_UNKNOWN: u32 = 0;
pub const SPEED_LOW: u32 = 1;
pub const SPEED_FULL: u32 = 2;
pub const SPEED_HIGH: u32 = 3;
pub const SPEED_WIRELESS: u32 = 4;
pub const SPEED_SUPER: u32 = 5;
pub const SPEED_SUPER_PLUS: u32 = 6;

/// Represents USBFS transfer speeds.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UsbfsSpeed {
    #[default]
    Unknown = SPEED_UNKNOWN,
    Low = SPEED_LOW,
    Full = SPEED_FULL,
    High = SPEED_HIGH,
    Wireless = SPEED_WIRELESS,
    Super = SPEED_SUPER,
    SuperPlus = SPEED_SUPER_PLUS,
}

impl UsbfsSpeed {
    /// Creates a new [UsbfsSpeed].
    pub const fn new() -> Self {
        Self::Unknown
    }

    /// Creates a new [UsbfsSpeed] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            SPEED_UNKNOWN => Self::Unknown,
            SPEED_LOW => Self::Low,
            SPEED_FULL => Self::Full,
            SPEED_HIGH => Self::High,
            SPEED_WIRELESS => Self::Wireless,
            SPEED_SUPER => Self::Super,
            SPEED_SUPER_PLUS => Self::SuperPlus,
            _ => Self::Unknown,
        }
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

impl From<u32> for UsbfsSpeed {
    fn from(val: u32) -> Self {
        Self::create(val)
    }
}

impl From<&UsbfsSpeed> for u32 {
    fn from(val: &UsbfsSpeed) -> Self {
        val.inner()
    }
}

impl From<UsbfsSpeed> for u32 {
    fn from(val: UsbfsSpeed) -> Self {
        val.into_inner()
    }
}

impl fmt::Display for UsbfsSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_speed() {
        assert_eq!(SPEED_UNKNOWN, UsbfsSpeed::Unknown.inner());
        assert_eq!(SPEED_LOW, UsbfsSpeed::Low.inner());
        assert_eq!(SPEED_FULL, UsbfsSpeed::Full.inner());
        assert_eq!(SPEED_HIGH, UsbfsSpeed::High.inner());
        assert_eq!(SPEED_WIRELESS, UsbfsSpeed::Wireless.inner());
        assert_eq!(SPEED_SUPER, UsbfsSpeed::Super.inner());
        assert_eq!(SPEED_SUPER_PLUS, UsbfsSpeed::SuperPlus.inner());

        assert_eq!(UsbfsSpeed::from(SPEED_UNKNOWN), UsbfsSpeed::Unknown);
        assert_eq!(UsbfsSpeed::from(SPEED_LOW), UsbfsSpeed::Low);
        assert_eq!(UsbfsSpeed::from(SPEED_FULL), UsbfsSpeed::Full);
        assert_eq!(UsbfsSpeed::from(SPEED_HIGH), UsbfsSpeed::High);
        assert_eq!(UsbfsSpeed::from(SPEED_WIRELESS), UsbfsSpeed::Wireless);
        assert_eq!(UsbfsSpeed::from(SPEED_SUPER), UsbfsSpeed::Super);
        assert_eq!(UsbfsSpeed::from(SPEED_SUPER_PLUS), UsbfsSpeed::SuperPlus);
    }
}
