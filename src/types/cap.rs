use std::fmt;

pub const CAP_ZERO_PACKET: u32 = 0x01;
pub const CAP_BULK_CONTINUATION: u32 = 0x02;
pub const CAP_NO_PACKET_SIZE_LIM: u32 = 0x04;
pub const CAP_BULK_SCATTER_GATHER: u32 = 0x08;
pub const CAP_REAP_AFTER_DISCONNECT: u32 = 0x10;

/// Represents USBFS capabilities.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UsbfsCap {
    None = 0x00,
    #[default]
    ZeroPacket = CAP_ZERO_PACKET,
    BulkContinuation = CAP_BULK_CONTINUATION,
    NoPacketSizeLim = CAP_NO_PACKET_SIZE_LIM,
    BulkScatterGather = CAP_BULK_SCATTER_GATHER,
    ReapAfterDisconnect = CAP_REAP_AFTER_DISCONNECT,
}

impl UsbfsCap {
    /// Creates a new [UsbfsCap].
    pub const fn new() -> Self {
        Self::ZeroPacket
    }

    /// Creates a new [UsbfsCap] from the provided parameter.
    pub const fn create(val: u32) -> Self {
        match val {
            CAP_ZERO_PACKET => Self::ZeroPacket,
            CAP_BULK_CONTINUATION => Self::BulkContinuation,
            CAP_NO_PACKET_SIZE_LIM => Self::NoPacketSizeLim,
            CAP_BULK_SCATTER_GATHER => Self::BulkScatterGather,
            CAP_REAP_AFTER_DISCONNECT => Self::ReapAfterDisconnect,
            _ => Self::None,
        }
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
            UsbfsCap::None => "none",
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

impl From<u32> for UsbfsCap {
    fn from(val: u32) -> Self {
        Self::create(val)
    }
}

impl From<&UsbfsCap> for u32 {
    fn from(val: &UsbfsCap) -> Self {
        val.inner()
    }
}

impl From<UsbfsCap> for u32 {
    fn from(val: UsbfsCap) -> Self {
        val.into_inner()
    }
}

impl fmt::Display for UsbfsCap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, r#""{}""#, <&str>::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_cap() {
        assert_eq!(CAP_ZERO_PACKET, UsbfsCap::ZeroPacket.inner());
        assert_eq!(CAP_BULK_CONTINUATION, UsbfsCap::BulkContinuation.inner());
        assert_eq!(CAP_NO_PACKET_SIZE_LIM, UsbfsCap::NoPacketSizeLim.inner());
        assert_eq!(CAP_BULK_SCATTER_GATHER, UsbfsCap::BulkScatterGather.inner());
        assert_eq!(
            CAP_REAP_AFTER_DISCONNECT,
            UsbfsCap::ReapAfterDisconnect.inner()
        );
        assert_eq!(0, UsbfsCap::None.inner());

        assert_eq!(UsbfsCap::from(CAP_ZERO_PACKET), UsbfsCap::ZeroPacket);
        assert_eq!(
            UsbfsCap::from(CAP_BULK_CONTINUATION),
            UsbfsCap::BulkContinuation
        );
        assert_eq!(
            UsbfsCap::from(CAP_NO_PACKET_SIZE_LIM),
            UsbfsCap::NoPacketSizeLim
        );
        assert_eq!(
            UsbfsCap::from(CAP_BULK_SCATTER_GATHER),
            UsbfsCap::BulkScatterGather
        );
        assert_eq!(
            UsbfsCap::from(CAP_REAP_AFTER_DISCONNECT),
            UsbfsCap::ReapAfterDisconnect
        );
        assert_eq!(UsbfsCap::from(0), UsbfsCap::None);
    }
}
