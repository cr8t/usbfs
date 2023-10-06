pub const SYSFS_MOUNT_PATH: &str = "/sys";
pub const SYSFS_DEVICE_PATH: &str = "/sys/bus/usb/devices";

pub const USBFS_MAX_DRIVER_NAME: usize = 255;
pub const USBFS_MAX_DRIVER_NAME_FFI: usize = 256;
pub const MAX_BULK_BUFFER_LENGTH: usize = 16384;
pub const MAX_CTRL_BUFFER_LENGTH: usize = 4096;
pub const MAX_ISO_PACKETS_PER_URB: usize = 128;
