use std::fmt;

/// Represents a USBFS interface setting.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UsbfsSetInterface {
    interface: u32,
    altsetting: u32,
}

impl UsbfsSetInterface {
    /// Creates a new [UsbfsSetInterface].
    pub const fn new() -> Self {
        Self {
            interface: 0,
            altsetting: 0,
        }
    }

    /// Gets the interface.
    pub const fn interface(&self) -> u32 {
        self.interface
    }

    /// Gets the altsetting.
    pub const fn altsetting(&self) -> u32 {
        self.altsetting
    }
}

impl fmt::Display for UsbfsSetInterface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""interface": {}, "#, self.interface)?;
        write!(f, r#""altsetting": {}"#, self.altsetting)?;
        write!(f, "}}")
    }
}
