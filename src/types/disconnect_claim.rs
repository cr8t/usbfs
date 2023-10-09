use std::fmt;

use super::DriverName;

pub const CLAIM_FLAG_NONE: u8 = 0x00;
pub const CLAIM_FLAG_IF_DRIVER: u8 = 0x01;
pub const CLAIM_FLAG_EXCEPT_DRIVER: u8 = 0x02;

/// Represents USBFS disconnect claim flags.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UsbfsDisconnectClaimFlag {
    None = CLAIM_FLAG_NONE,
    #[default]
    IfDriver = CLAIM_FLAG_IF_DRIVER,
    ExceptDriver = CLAIM_FLAG_EXCEPT_DRIVER,
}

impl UsbfsDisconnectClaimFlag {
    /// Creates a new [UsbfsDisconnectClaimFlag].
    pub const fn new() -> Self {
        Self::IfDriver
    }

    /// Creates a new [UsbfsDisconnectClaimFlag] from the provided parameter.
    pub const fn create(val: u8) -> Self {
        match val {
            CLAIM_FLAG_IF_DRIVER => Self::IfDriver,
            CLAIM_FLAG_EXCEPT_DRIVER => Self::ExceptDriver,
            _ => Self::None,
        }
    }

    /// Gets the inner representation of the [UsbfsDisconnectClaimFlag].
    pub const fn inner(&self) -> u8 {
        *self as u8
    }

    /// Converts into the inner representation of the [UsbfsDisconnectClaimFlag].
    pub fn into_inner(self) -> u8 {
        self as u8
    }
}

impl From<&UsbfsDisconnectClaimFlag> for &'static str {
    fn from(val: &UsbfsDisconnectClaimFlag) -> Self {
        match val {
            UsbfsDisconnectClaimFlag::None => "no driver",
            UsbfsDisconnectClaimFlag::IfDriver => "IF driver",
            UsbfsDisconnectClaimFlag::ExceptDriver => "except driver",
        }
    }
}

impl From<u8> for UsbfsDisconnectClaimFlag {
    fn from(val: u8) -> Self {
        Self::create(val)
    }
}

impl From<&UsbfsDisconnectClaimFlag> for u8 {
    fn from(val: &UsbfsDisconnectClaimFlag) -> Self {
        val.inner()
    }
}

impl From<UsbfsDisconnectClaimFlag> for u8 {
    fn from(val: UsbfsDisconnectClaimFlag) -> Self {
        val.into_inner()
    }
}

impl fmt::Display for UsbfsDisconnectClaimFlag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

/// Represents USBFS disconnect claim.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UsbfsDisconnectClaim {
    interface: u32,
    flags: UsbfsDisconnectClaimFlag,
    driver: DriverName,
}

impl UsbfsDisconnectClaim {
    /// Creates a new [UsbfsDisconnectClaim].
    pub const fn new() -> Self {
        Self {
            interface: 0,
            flags: UsbfsDisconnectClaimFlag::new(),
            driver: DriverName::new(),
        }
    }

    /// Creates a new [UsbfsDisconnectClaim] from the provided parameters.
    pub fn create(interface: u32, flags: UsbfsDisconnectClaimFlag, driver: &str) -> Self {
        Self {
            interface,
            flags,
            driver: driver.into(),
        }
    }

    /// Gets the interface number.
    pub const fn interface(&self) -> u32 {
        self.interface
    }

    /// Sets the interface number.
    pub fn set_interface(&mut self, iface: u32) {
        self.interface = iface;
    }

    /// Builder function that sets the interface number.
    pub fn with_interface(mut self, iface: u32) -> Self {
        self.set_interface(iface);
        self
    }

    /// Gets the [UsbfsDisconnectClaimFlag].
    pub const fn flags(&self) -> UsbfsDisconnectClaimFlag {
        self.flags
    }

    /// Sets the [UsbfsDisconnectClaimFlags].
    pub fn set_flags(&mut self, flags: UsbfsDisconnectClaimFlag) {
        self.flags = flags;
    }

    /// Builder function that sets the [UsbfsDisconnectClaimFlags].
    pub fn with_flags(mut self, flags: UsbfsDisconnectClaimFlag) -> Self {
        self.set_flags(flags);
        self
    }

    /// Gets the driver name.
    pub fn driver(&self) -> &str {
        self.driver.as_str()
    }

    /// Sets the driver name.
    pub fn set_driver(&mut self, driver: &str) {
        self.driver.from_string(driver);
    }

    /// Builder function that sets the driver name.
    pub fn with_driver(mut self, driver: &str) -> Self {
        self.set_driver(driver);
        self
    }
}

impl Default for UsbfsDisconnectClaim {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UsbfsDisconnectClaim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""interface": {}, "#, self.interface)?;
        write!(f, r#""flags": {}, "#, self.flags)?;
        write!(f, r#""driver": "{}""#, self.driver())?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::USBFS_MAX_DRIVER_NAME;

    #[test]
    fn test_usbfs_disconnect_claim_flag() {
        assert_eq!(CLAIM_FLAG_NONE, UsbfsDisconnectClaimFlag::None.inner());
        assert_eq!(
            CLAIM_FLAG_IF_DRIVER,
            UsbfsDisconnectClaimFlag::IfDriver.inner()
        );
        assert_eq!(
            CLAIM_FLAG_EXCEPT_DRIVER,
            UsbfsDisconnectClaimFlag::ExceptDriver.inner()
        );

        assert_eq!(
            UsbfsDisconnectClaimFlag::from(CLAIM_FLAG_NONE),
            UsbfsDisconnectClaimFlag::None
        );
        assert_eq!(
            UsbfsDisconnectClaimFlag::from(CLAIM_FLAG_IF_DRIVER),
            UsbfsDisconnectClaimFlag::IfDriver
        );
        assert_eq!(
            UsbfsDisconnectClaimFlag::from(CLAIM_FLAG_EXCEPT_DRIVER),
            UsbfsDisconnectClaimFlag::ExceptDriver
        );
    }

    #[test]
    fn test_usbfs_disconnect_claim() {
        let mut null_claim = UsbfsDisconnectClaim::new();

        let exp_interface = 1;
        let exp_flag = UsbfsDisconnectClaimFlag::ExceptDriver;
        let exp_driver = "test_driver";

        let exp_claim = UsbfsDisconnectClaim::create(exp_interface, exp_flag, exp_driver);

        assert_eq!(null_claim.interface(), 0);
        assert_eq!(null_claim.flags(), UsbfsDisconnectClaimFlag::new());
        assert_eq!(null_claim.driver(), "");

        null_claim.set_interface(exp_interface);
        assert_eq!(null_claim.interface(), exp_interface);

        null_claim.set_flags(exp_flag);
        assert_eq!(null_claim.flags(), exp_flag);

        null_claim.set_driver(exp_driver);
        assert_eq!(null_claim.driver(), exp_driver);

        assert_eq!(exp_claim.interface(), exp_interface);
        assert_eq!(exp_claim.flags(), exp_flag);
        assert_eq!(exp_claim.driver(), exp_driver);

        assert_eq!(
            UsbfsDisconnectClaim::new()
                .with_interface(exp_interface)
                .interface(),
            exp_interface
        );
        assert_eq!(
            UsbfsDisconnectClaim::new().with_flags(exp_flag).flags(),
            exp_flag
        );
        assert_eq!(
            UsbfsDisconnectClaim::new().with_driver(exp_driver).driver(),
            exp_driver
        );

        let over_max_driver = "a".repeat(USBFS_MAX_DRIVER_NAME + 2);
        let max_claim = UsbfsDisconnectClaim::new().with_driver(over_max_driver.as_str());

        assert_eq!(
            max_claim.driver(),
            &over_max_driver[..USBFS_MAX_DRIVER_NAME]
        );
        assert_eq!(max_claim.driver().len(), USBFS_MAX_DRIVER_NAME);
    }
}
