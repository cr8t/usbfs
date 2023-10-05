use super::*;

ioctl_readwrite!(usbfs_control, b'U', 0, UsbfsCtrlTransferFfi);
ioctl_read!(usbfs_setinterface, b'U', 4, UsbfsSetInterface); 
ioctl_read!(usbfs_setconfiguration, b'U', 5, u32);
ioctl_write_ptr!(usbfs_getdriver, b'U', 8, UsbfsGetDriver);
ioctl_read!(usbfs_submiturb, b'U', 10, UrbFfi);
ioctl_none!(usbfs_discardurb, b'U', 11);
ioctl_write_ptr!(usbfs_reapurbndelay, b'U', 13, UrbFfi);
ioctl_read!(usbfs_claiminterface, b'U', 15, u32);
ioctl_read!(usbfs_releaseinterface, b'U', 16, u32);
ioctl_write_ptr!(usbfs_connectinfo, b'U', 17, UsbfsConnectInfo);
ioctl_readwrite!(usbfs_ioctl, b'U', 18, UsbfsIoctlFfi);
ioctl_none!(usbfs_reset, b'U', 20);
ioctl_read!(usbfs_clear_halt, b'U', 21, u32);
ioctl_none!(usbfs_disconnect, b'U', 22);
ioctl_none!(usbfs_connect, b'U', 23);
ioctl_read!(usbfs_get_capabilities, b'U', 26, u32);
ioctl_read!(usbfs_disconnect_claim, b'U', 27, UsbfsDisconnectClaim);
ioctl_read!(usbfs_alloc_streams, b'U', 28, UsbfsStreamsFfi);
ioctl_read!(usbfs_free_streams, b'U', 29, UsbfsStreamsFfi);
ioctl_write_int!(usbfs_drop_privileges, b'U', 30);
ioctl_none!(usbfs_get_speed, b'U', 31);
