use std::{ffi::c_void, fmt};

/// Convenience trait for passing data arguments to USBFS IOCTL drivers.
pub trait UsbfsIoctlData: std::fmt::Debug {
    fn as_raw_ptr(&self) -> *const c_void;
    fn as_raw_ptr_mut(&mut self) -> *mut c_void;
}

impl UsbfsIoctlData for [u8] {
    fn as_raw_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }

    fn as_raw_ptr_mut(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *mut _
    }
}

impl UsbfsIoctlData for &mut [u8] {
    fn as_raw_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }

    fn as_raw_ptr_mut(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *mut _
    }
}

impl UsbfsIoctlData for Vec<u8> {
    fn as_raw_ptr(&self) -> *const c_void {
        self.as_ptr() as *const _
    }

    fn as_raw_ptr_mut(&mut self) -> *mut c_void {
        self.as_mut_ptr() as *mut _
    }
}

impl PartialEq for dyn UsbfsIoctlData {
    fn eq(&self, rhs: &Self) -> bool {
        self.as_raw_ptr() == rhs.as_raw_ptr()
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
pub struct UsbfsIoctl<'a> {
    ifno: i32,
    ioctl_code: i32,
    data: Option<&'a mut dyn UsbfsIoctlData>,
}

impl<'a> UsbfsIoctl<'a> {
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

    /// Gets the data argument as a mutable FFI pointer.
    ///
    /// **SAFETY**: the result may be null if there is no `data` argument.
    /// Users should not directly dereference the pointer.
    pub fn data_ptr(&self) -> *const c_void {
        if let Some(data) = self.data.as_ref() {
            data.as_raw_ptr()
        } else {
            std::ptr::null()
        }
    }

    /// Gets the data argument as a mutable FFI pointer.
    ///
    /// **SAFETY**: the result may be null if there is no `data` argument.
    /// Users should not directly dereference the pointer.
    pub fn data_ptr_mut(&mut self) -> *mut c_void {
        if let Some(data) = self.data.as_mut() {
            data.as_raw_ptr_mut()
        } else {
            std::ptr::null_mut()
        }
    }

    /// Sets the data argument.
    pub fn set_data(&mut self, data: &'a mut dyn UsbfsIoctlData) {
        self.data.replace(data);
    }

    /// Builder function that sets the data argument.
    pub fn with_data(mut self, data: &'a mut dyn UsbfsIoctlData) -> Self {
        self.set_data(data);
        self
    }

    /// Unsets the data argument.
    pub fn unset_data(&mut self) {
        self.data.take();
    }
}

impl<'a> PartialEq for UsbfsIoctl<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        self.ifno == rhs.ifno
            && self.ioctl_code == rhs.ioctl_code
            && match (self.data.as_ref(), rhs.data.as_ref()) {
                (Some(d), Some(rd)) => d.as_raw_ptr() == rd.as_raw_ptr(),
                (None, None) => true,
                _ => false,
            }
    }
}

impl<'a> fmt::Display for UsbfsIoctl<'a> {
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

impl<'a> From<&mut UsbfsIoctl<'a>> for UsbfsIoctlFfi {
    fn from(val: &mut UsbfsIoctl<'a>) -> Self {
        Self {
            ifno: val.ifno,
            ioctl_code: val.ioctl_code,
            data: val.data_ptr_mut(),
        }
    }
}

impl Default for UsbfsIoctlFfi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_ioctl_data() {
        let mut exp_data = [0u8; 128];
        let exp_ptr = exp_data.as_mut_ptr() as usize;

        assert_eq!(
            UsbfsIoctlData::as_raw_ptr(exp_data.as_mut()) as usize,
            exp_ptr
        );
    }

    #[test]
    fn test_usbfs_ioctl() {
        let exp_ifno = 1;
        let exp_ioctl_code = 2;
        let mut exp_data = [42u8; 128];
        let exp_data_ptr = exp_data.as_ptr() as usize;
        let mut exp_data_mut = exp_data.as_mut();

        let mut exp_ioctl = UsbfsIoctl::new()
            .with_ifno(exp_ifno)
            .with_ioctl_code(exp_ioctl_code)
            .with_data(&mut exp_data_mut);

        assert_eq!(exp_ioctl.ifno(), exp_ifno);
        assert_eq!(exp_ioctl.ioctl_code(), exp_ioctl_code);
        assert_eq!(exp_ioctl.data_ptr() as usize, exp_data_ptr);

        let mut null_ioctl = UsbfsIoctl::new();

        assert_eq!(null_ioctl.ifno(), 0);
        assert_eq!(null_ioctl.ioctl_code(), 0);
        assert_eq!(null_ioctl.data_ptr(), std::ptr::null());

        null_ioctl.set_ifno(exp_ifno);
        null_ioctl.set_ioctl_code(exp_ioctl_code);
        exp_ioctl.unset_data();

        assert_eq!(null_ioctl, exp_ioctl);
    }
}
