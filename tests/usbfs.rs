use usbfs::*;

// FIXME: currently returns a constant, it should dynamically detect a valid USB file descriptor.
//
// Most relevant is the CI context, where a pseudo-device needs to be mocked for deterministic tests.
fn get_usb_fd() -> i32 {
    0
}

#[test]
fn test_control() -> Result<()> {
    let fd = get_usb_fd();
    let mut control = UsbfsCtrlTransfer::new()
        // ENDPOINT_IN
        .with_request_type(0x80)
        // USB_GET_CONFIGURATION
        .with_request(0x08)
        .with_timeout(1000)
        .with_data([0u8]);

    usbfs_control(fd, &mut control).ok();

    Ok(())
}

#[test]
fn test_set_release_iface() -> Result<()> {
    let fd = get_usb_fd();
    let mut set_iface = UsbfsSetInterface::create(1, 0);

    usbfs_set_interface(fd, &mut set_iface).ok();
    usbfs_claim_interface(fd, set_iface.interface_mut()).ok();
    usbfs_release_interface(fd, set_iface.interface_mut()).ok();

    Ok(())
}

#[test]
fn test_set_configuration() -> Result<()> {
    let fd = get_usb_fd();
    let mut config = 0u32;

    usbfs_set_configuration(fd, &mut config).ok();

    Ok(())
}

#[test]
fn test_get_driver() -> Result<()> {
    let fd = get_usb_fd();
    let mut driver = UsbfsGetDriver::new().with_interface(0);

    usbfs_get_driver(fd, &mut driver).ok();

    Ok(())
}

#[test]
fn test_submit_urb() -> Result<()> {
    let fd = get_usb_fd();
    let mut urb = Urb::new()
        // URB_TYPE_CONTROL
        .with_urb_type(2)
        .with_buffer([0; 4]);

    usbfs_submit_urb(fd, &mut urb).ok();

    Ok(())
}

#[test]
fn test_discard_urb() -> Result<()> {
    let fd = get_usb_fd();

    usbfs_discard_urb(fd).ok();

    Ok(())
}

#[test]
fn test_reap_urb_ndelay() -> Result<()> {
    let fd = get_usb_fd();
    let mut urb = Urb::new()
        // URB_TYPE_CONTROL
        .with_urb_type(2)
        .with_buffer([0; 4]);

    usbfs_reap_urb_ndelay(fd, &mut urb).ok();

    Ok(())
}

#[test]
fn test_connect_info() -> Result<()> {
    let fd = get_usb_fd();
    let mut info = UsbfsConnectInfo::create(1, 42);

    usbfs_connect_info(fd, &mut info).ok();

    Ok(())
}

#[test]
fn test_ioctl() -> Result<()> {
    let fd = get_usb_fd();
    let mut ioctl = UsbfsIoctl::new().with_ifno(1).with_ioctl_code(18);

    usbfs_ioctl(fd, &mut ioctl).ok();

    Ok(())
}

#[test]
fn test_reset() -> Result<()> {
    let fd = get_usb_fd();

    usbfs_reset(fd).ok();

    Ok(())
}

#[test]
fn test_clear_halt() -> Result<()> {
    let fd = get_usb_fd();
    let mut iface = 1u32;

    usbfs_clear_halt(fd, &mut iface).ok();

    Ok(())
}

#[test]
fn test_disconnect() -> Result<()> {
    let fd = get_usb_fd();

    usbfs_disconnect(fd).ok();

    Ok(())
}

#[test]
fn test_connect() -> Result<()> {
    let fd = get_usb_fd();

    usbfs_connect(fd).ok();

    Ok(())
}

#[test]
fn test_get_capabilities() -> Result<()> {
    let fd = get_usb_fd();
    let mut iface = 1u32;

    usbfs_get_capabilities(fd, &mut iface).ok();

    Ok(())
}

#[test]
fn test_disconnect_claim() -> Result<()> {
    let fd = get_usb_fd();
    let mut claim = UsbfsDisconnectClaim::new();

    usbfs_disconnect_claim(fd, &mut claim).ok();

    Ok(())
}

#[test]
fn test_alloc_streams() -> Result<()> {
    let fd = get_usb_fd();
    let mut streams = UsbfsStreams::new();

    usbfs_alloc_streams(fd, &mut streams).ok();

    Ok(())
}

#[test]
fn test_free_streams() -> Result<()> {
    let fd = get_usb_fd();
    let mut streams = UsbfsStreams::new();

    usbfs_free_streams(fd, &mut streams).ok();

    Ok(())
}

#[test]
fn test_drop_privileges() -> Result<()> {
    let fd = get_usb_fd();
    let privileges = 0u64;

    usbfs_drop_privileges(fd, privileges).ok();

    Ok(())
}

#[test]
fn test_get_speed() -> Result<()> {
    let fd = get_usb_fd();

    usbfs_get_speed(fd).ok();

    Ok(())
}
