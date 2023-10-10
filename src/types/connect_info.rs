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
        Self { devnum: 0, slow: 0 }
    }

    /// Creates a new [UsbfsConnectInfo] from the provided parameter.
    pub const fn create(devnum: u32, slow: u8) -> Self {
        Self { devnum, slow }
    }

    /// Gets the device number.
    pub const fn devnum(&self) -> u32 {
        self.devnum
    }

    /// Sets the device number.
    pub fn set_devnum(&mut self, devnum: u32) {
        self.devnum = devnum;
    }

    /// Builder function that sets the device number.
    pub fn with_devnum(mut self, devnum: u32) -> Self {
        self.set_devnum(devnum);
        self
    }

    /// Gets the slow field.
    pub const fn slow(&self) -> u8 {
        self.slow
    }

    /// Sets the slow number.
    pub fn set_slow(&mut self, slow: u8) {
        self.slow = slow;
    }

    /// Builder function that sets the slow number.
    pub fn with_slow(mut self, slow: u8) -> Self {
        self.set_slow(slow);
        self
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_connect_info() {
        let exp_devnum = 42;
        let exp_slow = 1;

        let exp_info = UsbfsConnectInfo::create(exp_devnum, exp_slow);
        let mut null_info = UsbfsConnectInfo::new();

        assert_eq!(exp_info.devnum(), exp_devnum);
        assert_eq!(exp_info.slow(), exp_slow);

        assert_eq!(null_info.devnum(), 0);
        assert_eq!(null_info.slow(), 0);

        null_info.set_devnum(exp_devnum);
        assert_eq!(null_info.devnum(), exp_devnum);

        null_info.set_slow(exp_slow);
        assert_eq!(null_info.slow(), exp_slow);

        assert_eq!(
            UsbfsConnectInfo::new().with_devnum(exp_devnum).devnum(),
            exp_devnum
        );
        assert_eq!(UsbfsConnectInfo::new().with_slow(exp_slow).slow(), exp_slow);
    }
}
