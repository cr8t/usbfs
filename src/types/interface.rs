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

    /// Creates a new [UsbfsSetInterface] from the provided parameters.
    pub const fn create(interface: u32, altsetting: u32) -> Self {
        Self {
            interface,
            altsetting,
        }
    }

    /// Gets the interface.
    pub const fn interface(&self) -> u32 {
        self.interface
    }

    /// Gets a mutable reference to the interface.
    pub fn interface_mut(&mut self) -> &mut u32 {
        &mut self.interface
    }

    /// Sets the interface.
    pub fn set_interface(&mut self, interface: u32) {
        self.interface = interface;
    }

    /// Sets the interface.
    pub fn with_interface(mut self, interface: u32) -> Self {
        self.set_interface(interface);
        self
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
