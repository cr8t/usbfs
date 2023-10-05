use std::{cmp, fmt};

use crate::USBFS_MAX_DRIVER_NAME;

/// Represents USBFS disconnect claim flags.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UsbfsDisconnectClaimFlag {
    #[default]
    IfDriver = 0x01,
    ExceptDriver = 0x02,
}

impl UsbfsDisconnectClaimFlag {
    /// Creates a new [UsbfsDisconnectClaimFlag].
    pub const fn new() -> Self {
        Self::IfDriver
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
            UsbfsDisconnectClaimFlag::IfDriver => "IF driver",
            UsbfsDisconnectClaimFlag::ExceptDriver => "except driver",
        }
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
    driver: [u8; USBFS_MAX_DRIVER_NAME + 1],
}

impl UsbfsDisconnectClaim {
    /// Creates a new [UsbfsDisconnectClaim].
    pub const fn new() -> Self {
        Self {
            interface: 0,
            flags: UsbfsDisconnectClaimFlag::new(),
            driver: [0u8; USBFS_MAX_DRIVER_NAME + 1],
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
        std::str::from_utf8(self.driver.as_ref()).unwrap_or("")
    }

    /// Sets the driver name.
    pub fn set_driver(&mut self, driver: &str) {
        let dbytes = driver.as_bytes();
        let len = cmp::min(dbytes.len(), USBFS_MAX_DRIVER_NAME);

        self.driver[..len].copy_from_slice(&dbytes[..len]);
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
