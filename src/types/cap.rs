use std::fmt;

/// Represents USBFS capabilities.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UsbfsCap {
    #[default]
    ZeroPacket = 0x01,
    BulkContinuation = 0x02,
    NoPacketSizeLim = 0x04,
    BulkScatterGather = 0x08,
    ReapAfterDisconnect = 0x10,
}

impl UsbfsCap {
    /// Creates a new [UsbfsCap].
    pub const fn new() -> Self {
        Self::ZeroPacket
    }

    /// Gets the inner representation of the [UsbfsCap].
    pub const fn inner(&self) -> u32 {
        *self as u32
    }

    /// Converts into the inner representation of the [UsbfsCap].
    pub fn into_inner(self) -> u32 {
        self as u32
    }
}

impl From<&UsbfsCap> for &'static str {
    fn from(val: &UsbfsCap) -> Self {
        match val {
            UsbfsCap::ZeroPacket => "zero packet",
            UsbfsCap::BulkContinuation => "bulk continuation",
            UsbfsCap::NoPacketSizeLim => "no packet size limit",
            UsbfsCap::BulkScatterGather => "bulk scatter gather",
            UsbfsCap::ReapAfterDisconnect => "reap after disconnect",
        }
    }
}

impl From<UsbfsCap> for &'static str {
    fn from(val: UsbfsCap) -> Self {
        (&val).into()
    }
}

impl fmt::Display for UsbfsCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}
