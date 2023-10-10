use std::fmt;

/// Represents a USBFS Isochronous packet description.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UsbfsIsoPacketDesc {
    length: u32,
    actual_length: u32,
    status: u32,
}

impl UsbfsIsoPacketDesc {
    /// Creates a new [UsbfsIsoPacketDesc].
    pub const fn new() -> Self {
        Self {
            length: 0,
            actual_length: 0,
            status: 0,
        }
    }

    /// Gets the length.
    pub const fn length(&self) -> u32 {
        self.length
    }

    /// Sets the length.
    pub fn set_length(&mut self, length: u32) {
        self.length = length;
    }

    /// Builder function that sets the length.
    pub fn with_length(mut self, length: u32) -> Self {
        self.set_length(length);
        self
    }

    /// Gets the actual length.
    pub const fn actual_length(&self) -> u32 {
        self.actual_length
    }

    /// Sets the actual length.
    pub fn set_actual_length(&mut self, actual_length: u32) {
        if actual_length <= self.length {
            self.actual_length = actual_length;
        }
    }

    /// Builder function that sets the actual length.
    pub fn with_actual_length(mut self, actual_length: u32) -> Self {
        self.set_actual_length(actual_length);
        self
    }

    /// Gets the status.
    pub const fn status(&self) -> u32 {
        self.status
    }

    /// Sets the status.
    pub fn set_status(&mut self, status: u32) {
        self.status = status;
    }

    /// Builder function that sets the status.
    pub fn with_status(mut self, status: u32) -> Self {
        self.set_status(status);
        self
    }
}

impl fmt::Display for UsbfsIsoPacketDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""length": {}, "#, self.length)?;
        write!(f, r#""actual_length": {}, "#, self.actual_length)?;
        write!(f, r#""status": {}"#, self.status)?;
        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_iso_packet_desc() {
        let exp_length = 1;
        let exp_actual_length = 1;
        let exp_status = 2;

        let exp_desc = UsbfsIsoPacketDesc::new()
            .with_length(exp_length)
            .with_actual_length(exp_actual_length)
            .with_status(exp_status);

        let mut null_desc = UsbfsIsoPacketDesc::new();

        assert_eq!(exp_desc.length(), exp_length);
        assert_eq!(exp_desc.actual_length(), exp_actual_length);
        assert_eq!(exp_desc.status(), exp_status);

        assert_eq!(null_desc.length(), 0);
        assert_eq!(null_desc.actual_length(), 0);
        assert_eq!(null_desc.status(), 0);

        null_desc.set_length(exp_length);
        assert_eq!(null_desc.length(), exp_length);

        null_desc.set_actual_length(exp_actual_length);
        assert_eq!(null_desc.actual_length(), exp_actual_length);

        null_desc.set_status(exp_status);
        assert_eq!(null_desc.status(), exp_status);

        assert_eq!(null_desc, exp_desc);
    }
}
