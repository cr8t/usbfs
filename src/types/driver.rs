use std::{cmp, fmt};

use crate::USBFS_MAX_DRIVER_NAME;

/// Represents an argument to get a USBFS driver from an `ioctl`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UsbfsGetDriver {
    interface: u32,
    driver: [u8; USBFS_MAX_DRIVER_NAME + 1],
}

impl UsbfsGetDriver {
    /// Creates a new [UbsfGetDriver].
    pub const fn new() -> Self {
        Self {
            interface: 0,
            driver: [0u8; USBFS_MAX_DRIVER_NAME + 1],
        }
    }

    /// Gets the interface.
    pub const fn interface(&self) -> u32 {
        self.interface
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

impl Default for UsbfsGetDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UsbfsGetDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""interface": {}, "#, self.interface)?;
        write!(f, r#""driver": "{}""#, self.driver())?;
        write!(f, "}}")
    }
}
