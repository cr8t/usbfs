use std::{cmp, fmt};

use crate::{USBFS_MAX_DRIVER_NAME, USBFS_MAX_DRIVER_NAME_FFI};

/// Represents a USBFS driver name.
///
/// The internal representation is a null-terminated byte buffer for C-like strings.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DriverName([u8; USBFS_MAX_DRIVER_NAME_FFI]);

impl DriverName {
    /// Creates a new [DriverName].
    pub const fn new() -> Self {
        Self([0u8; USBFS_MAX_DRIVER_NAME_FFI])
    }

    /// Creates a [DriverName] from a string.
    ///
    /// **NOTE** Sets at most [`USBFS_MAX_DRIVER_NAME`] bytes.
    pub fn from_string(&mut self, driver: &str) {
        let bytes = driver.as_bytes();
        // always leave the last byte null for C-like null-terminated strings
        let len = cmp::min(bytes.len(), USBFS_MAX_DRIVER_NAME);

        self.0[..len].copy_from_slice(&bytes[..len]);
        self.0[len..].copy_from_slice(&[0u8; USBFS_MAX_DRIVER_NAME_FFI][len..]);
    }

    /// Gets a [`&str`] representation of the [DriverName].
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(self.0.as_ref()).unwrap_or("")
    }
}

impl From<&str> for DriverName {
    fn from(val: &str) -> Self {
        let mut ret = Self::new();
        ret.from_string(val);
        ret
    }
}

impl Default for DriverName {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DriverName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, self.as_str())
    }
}

/// Represents an argument to get a USBFS driver from an `ioctl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UsbfsGetDriver {
    interface: u32,
    driver: DriverName,
}

impl UsbfsGetDriver {
    /// Creates a new [UbsfGetDriver].
    pub const fn new() -> Self {
        Self {
            interface: 0,
            driver: DriverName::new(),
        }
    }

    /// Creates a new [UsbfsGetDriver] from the provided parameters.
    pub fn create(interface: u32, driver: &str) -> Self {
        Self {
            interface,
            driver: DriverName::from(driver),
        }
    }

    /// Gets the interface.
    pub const fn interface(&self) -> u32 {
        self.interface
    }

    /// Sets the interface.
    pub fn set_interface(&mut self, interface: u32) {
        self.interface = interface;
    }

    /// Builder function that sets the interface.
    pub fn with_interface(mut self, interface: u32) -> Self {
        self.set_interface(interface);
        self
    }

    /// Gets the driver name.
    pub fn driver(&self) -> &str {
        self.driver.as_str()
    }

    /// Sets the driver name.
    ///
    /// **NOTE** Sets at most [`USBFS_MAX_DRIVER_NAME`] bytes.
    pub fn set_driver(&mut self, driver: &str) {
        self.driver.from_string(driver);
    }

    /// Builder function that sets the driver name.
    ///
    /// **NOTE** Sets at most [`USBFS_MAX_DRIVER_NAME`] bytes.
    pub fn with_driver(mut self, driver: &str) -> Self {
        self.set_driver(driver);
        self
    }
}

impl Default for UsbfsGetDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UsbfsGetDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""interface": {}, "#, self.interface)?;
        write!(f, r#""driver": {}"#, self.driver)?;
        write!(f, "}}")
    }
}
