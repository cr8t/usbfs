#[macro_use]
extern crate nix;

mod constants;
mod error;
mod ioctl;
mod types;

pub use constants::*;
pub use error::*;

pub use types::cap::UsbfsCap;
pub use types::connect_info::UsbfsConnectInfo;
pub use types::ctrl_transfer::UsbfsCtrlTransfer;
pub use types::disconnect_claim::{UsbfsDisconnectClaim, UsbfsDisconnectClaimFlag};
pub use types::driver::{DriverName, UsbfsGetDriver};
pub use types::interface::UsbfsSetInterface;
pub use types::ioctl::{UsbfsIoctl, UsbfsIoctlData};
pub use types::iso_packet_desc::UsbfsIsoPacketDesc;
pub use types::speed::UsbfsSpeed;
pub use types::streams::UsbfsStreams;
pub use types::urb::{TransferInfo, Urb, UrbUserContext};

use types::{UrbFfi, UsbfsCtrlTransferFfi, UsbfsIoctlFfi, UsbfsStreamsFfi};

/// USBFS Control transfer.
///
/// The user is responsible for setting all the relevant [UsbfsCtrlTransfer] fields.
pub fn usbfs_control(fd: i32, ctrl: &mut UsbfsCtrlTransfer) -> Result<()> {
    let mut ctrl = UsbfsCtrlTransferFfi::from(ctrl);
    unsafe {
        ioctl::usbfs_control(fd, &mut ctrl)?;
    }
    Ok(())
}

/// USBFS Set Interface
///
/// The user is responsible for setting all the relevant [UsbfsSetInterface] fields.
pub fn usbfs_set_interface(fd: i32, set_interface: &mut UsbfsSetInterface) -> Result<()> {
    unsafe {
        ioctl::usbfs_setinterface(fd, set_interface)?;
    }
    Ok(())
}

/// USBFS Set Configuration
pub fn usbfs_set_configuration(fd: i32, config: &mut u32) -> Result<()> {
    unsafe {
        ioctl::usbfs_setconfiguration(fd, config)?;
    }
    Ok(())
}

/// USBFS Get Driver
///
/// The user is responsible for setting all the relevant [UsbfsGetDriver] fields.
pub fn usbfs_get_driver(fd: i32, get_driver: &mut UsbfsGetDriver) -> Result<()> {
    unsafe {
        ioctl::usbfs_getdriver(fd, get_driver)?;
    }
    Ok(())
}

/// USBFS Submit URB
///
/// The user is responsible for setting all the relevant [Urb] fields.
pub fn usbfs_submit_urb(fd: i32, urb: &mut Urb) -> Result<()> {
    let mut urb_ffi = UrbFfi::from(urb);
    unsafe {
        ioctl::usbfs_submiturb(fd, &mut urb_ffi)?;
    }
    Ok(())
}

/// USBFS Discard URB
pub fn usbfs_discard_urb(fd: i32) -> Result<()> {
    unsafe {
        ioctl::usbfs_discardurb(fd)?;
    }
    Ok(())
}

/// USBFS Reap URB N_Delay
///
/// The user is responsible for setting all the relevant [Urb] fields.
pub fn usbfs_reap_urb_ndelay(fd: i32, urb: &mut Urb) -> Result<()> {
    let urb_ffi = UrbFfi::from(urb);
    unsafe {
        ioctl::usbfs_reapurbndelay(fd, &urb_ffi)?;
    }
    Ok(())
}

/// USBFS Claim Interface
pub fn usbfs_claim_interface(fd: i32, iface: &mut u32) -> Result<()> {
    unsafe {
        ioctl::usbfs_claiminterface(fd, iface)?;
    }
    Ok(())
}

/// USBFS Release Interface
pub fn usbfs_release_interface(fd: i32, iface: &mut u32) -> Result<()> {
    unsafe {
        ioctl::usbfs_releaseinterface(fd, iface)?;
    }
    Ok(())
}

/// USBFS Connect Info
///
/// The user is responsible for setting all the relevant [UsbfsConnectInfo] fields.
pub fn usbfs_connect_info(fd: i32, info: &mut UsbfsConnectInfo) -> Result<()> {
    unsafe {
        ioctl::usbfs_connectinfo(fd, info)?;
    }
    Ok(())
}

/// USBFS IOCTL
///
/// The user is responsible for setting all the relevant [UsbfsIoctl] fields.
pub fn usbfs_ioctl(fd: i32, ioctl: &mut UsbfsIoctl) -> Result<()> {
    let mut ioctl_ffi = UsbfsIoctlFfi::from(ioctl);
    unsafe {
        ioctl::usbfs_ioctl(fd, &mut ioctl_ffi)?;
    }
    Ok(())
}

/// USBFS Reset
pub fn usbfs_reset(fd: i32) -> Result<()> {
    unsafe {
        ioctl::usbfs_reset(fd)?;
    }
    Ok(())
}

/// USBFS Clear Halt
pub fn usbfs_clear_halt(fd: i32, iface: &mut u32) -> Result<()> {
    unsafe {
        ioctl::usbfs_clear_halt(fd, iface)?;
    }
    Ok(())
}

/// USBFS Disconnect
pub fn usbfs_disconnect(fd: i32) -> Result<()> {
    unsafe {
        ioctl::usbfs_disconnect(fd)?;
    }
    Ok(())
}

/// USBFS Connect
pub fn usbfs_connect(fd: i32) -> Result<()> {
    unsafe {
        ioctl::usbfs_connect(fd)?;
    }
    Ok(())
}

/// USBFS Get Capabilities
pub fn usbfs_get_capabilities(fd: i32, iface: &mut u32) -> Result<()> {
    unsafe {
        ioctl::usbfs_get_capabilities(fd, iface)?;
    }
    Ok(())
}

/// USBFS Disconnect Claim
///
/// The user is responsible for setting all the relevant [UsbfsDisconnectClaim] fields.
pub fn usbfs_disconnect_claim(fd: i32, claim: &mut UsbfsDisconnectClaim) -> Result<()> {
    unsafe {
        ioctl::usbfs_disconnect_claim(fd, claim)?;
    }
    Ok(())
}

/// USBFS Alloc Streams
///
/// Requests the kernel to allocate `num_streams` of USB packet streams.
///
/// The user is responsible for setting all the relevant [UsbfsStreams] fields.
pub fn usbfs_alloc_streams(fd: i32, streams: &mut UsbfsStreams) -> Result<()> {
    let mut streams_ffi = UsbfsStreamsFfi::from(streams);
    unsafe {
        ioctl::usbfs_alloc_streams(fd, &mut streams_ffi)?;
    }
    Ok(())
}

/// USBFS Free Streams
///
/// Requests the kernel to free `num_streams` of USB packet streams.
///
/// The user is responsible for setting all the relevant [UsbfsStreams] fields.
pub fn usbfs_free_streams(fd: i32, streams: &mut UsbfsStreams) -> Result<()> {
    let mut streams_ffi = UsbfsStreamsFfi::from(streams);
    unsafe {
        ioctl::usbfs_free_streams(fd, &mut streams_ffi)?;
    }
    Ok(())
}

/// USBFS Drop Privileges
pub fn usbfs_drop_privileges(fd: i32, privileges: u64) -> Result<()> {
    unsafe {
        ioctl::usbfs_drop_privileges(fd, privileges)?;
    }
    Ok(())
}

/// USBFS Get Speed
pub fn usbfs_get_speed(fd: i32) -> Result<()> {
    unsafe {
        ioctl::usbfs_get_speed(fd)?;
    }
    Ok(())
}
