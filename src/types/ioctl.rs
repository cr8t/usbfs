use std::{ffi::c_void, fmt};

/// Convenience trait for passing data arguments to USBFS IOCTL drivers.
pub trait UsbfsIoctlData: std::fmt::Debug {
    fn as_raw_ptr(&mut self) -> *mut c_void;
}

impl UsbfsIoctlData for &mut [u8] {
    fn as_raw_ptr(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *mut _
    }
}

impl UsbfsIoctlData for Vec<u8> {
    fn as_raw_ptr(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *mut _
    }
}

impl fmt::Display for dyn UsbfsIoctlData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// Represents USBFS `ioctl` information.
#[repr(C)]
#[derive(Debug, Default)]
pub struct UsbfsIoctl {
    ifno: i32,
    ioctl_code: i32,
    data: Option<Box<dyn UsbfsIoctlData>>,
}

impl UsbfsIoctl {
    /// Creates a new [UsbfsIoctl].
    pub fn new() -> Self {
        Self {
            ifno: 0,
            ioctl_code: 0,
            data: None,
        }
    }

    /// Gets the interface number.
    pub const fn ifno(&self) -> i32 {
        self.ifno
    }

    /// Sets the interface number.
    pub fn set_ifno(&mut self, ifno: i32) {
        self.ifno = ifno;
    }

    /// Builder function that sets the interface number.
    pub fn with_ifno(mut self, ifno: i32) -> Self {
        self.set_ifno(ifno);
        self
    }

    /// Gets the IOCTL code number.
    pub const fn ioctl_code(&self) -> i32 {
        self.ioctl_code
    }

    /// Sets the IOCTL code number.
    pub fn set_ioctl_code(&mut self, ioctl_code: i32) {
        self.ioctl_code = ioctl_code;
    }

    /// Builder function that sets the IOCTL code number.
    pub fn with_ioctl_code(mut self, ioctl_code: i32) -> Self {
        self.set_ioctl_code(ioctl_code);
        self
    }

    /// Gets the data argument.
    pub fn data(&mut self) -> Option<&dyn UsbfsIoctlData> {
        self.data.as_ref().map(|d| d.as_ref())
    }

    /// Gets the data argument as a mutable FFI pointer.
    ///
    /// **SAFETY**: the result may be null if there is no `data` argument.
    /// Users should not directly dereference the pointer.
    pub fn data_ptr(&mut self) -> *mut c_void {
        if let Some(data) = self.data.as_mut() {
            data.as_raw_ptr()
        } else {
            std::ptr::null_mut()
        }
    }

    /// Sets the data argument.
    pub fn set_data(&mut self, data: Box<dyn UsbfsIoctlData>) {
        self.data = Some(data);
    }

    /// Builder function that sets the data argument.
    pub fn with_data(mut self, data: Box<dyn UsbfsIoctlData>) -> Self {
        self.set_data(data);
        self
    }

    /// Unsets the data argument.
    pub fn unset_data(&mut self) -> Option<Box<dyn UsbfsIoctlData>> {
        self.data.take()
    }
}

impl fmt::Display for UsbfsIoctl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""ifno": {}, "#, self.ifno)?;
        write!(f, r#""ioctl_code": {}"#, self.ioctl_code)?;
        write!(f, "}}")
    }
}

/// Represents USBFS `ioctl` information passed to an FFI function.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct UsbfsIoctlFfi {
    ifno: i32,
    ioctl_code: i32,
    data: *mut c_void,
}

impl UsbfsIoctlFfi {
    pub const fn new() -> Self {
        Self {
            ifno: 0,
            ioctl_code: 0,
            data: std::ptr::null_mut(),
        }
    }
}

impl From<&mut UsbfsIoctl> for UsbfsIoctlFfi {
    fn from(val: &mut UsbfsIoctl) -> Self {
        Self {
            ifno: val.ifno,
            ioctl_code: val.ioctl_code,
            data: val.data_ptr(),
        }
    }
}

impl Default for UsbfsIoctlFfi {
    fn default() -> Self {
        Self::new()
    }
}
