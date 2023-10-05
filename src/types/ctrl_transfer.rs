use std::ffi::c_void;

/// Represents a USBFS Control transfer passed to `ioctl` FFI
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct UsbfsCtrlTransferFfi {
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
    timeout: u32,
    data: *mut c_void,
}

impl UsbfsCtrlTransferFfi {
    /// Creates a new [UsbfsCtrlTransferFfi].
    pub const fn new() -> Self {
        Self {
            bm_request_type: 0,
            b_request: 0,
            w_value: 0,
            w_index: 0,
            w_length: 0,
            timeout: 0,
            data: std::ptr::null_mut(),
        }
    }
}

impl Default for UsbfsCtrlTransferFfi {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&mut UsbfsCtrlTransfer> for UsbfsCtrlTransferFfi {
    fn from(val: &mut UsbfsCtrlTransfer) -> Self {
        Self {
            bm_request_type: val.bm_request_type,
            b_request: val.b_request,
            w_value: val.w_value,
            w_index: val.w_index,
            w_length: val.data.len() as u16,
            timeout: val.timeout,
            data: val.data.as_mut_ptr() as *mut _,
        }
    }
}

/// Represents a USBFS Control transfer
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UsbfsCtrlTransfer {
    bm_request_type: u8,
    b_request: u8,
    w_value: u16,
    w_index: u16,
    w_length: u16,
    timeout: u32,
    data: Vec<u8>,
}

impl UsbfsCtrlTransfer {
    /// Creates a new [UsbfsCtrlTransfer].
    pub fn new() -> Self {
        Self {
            bm_request_type: 0,
            b_request: 0,
            w_value: 0,
            w_index: 0,
            w_length: 0,
            timeout: 0,
            data: Vec::new(),
        }
    }

    /// Gets the request type.
    pub const fn request_type(&self) -> u8 {
        self.bm_request_type
    }

    /// Gets the request.
    pub const fn request(&self) -> u8 {
        self.b_request
    }

    /// Gets the value.
    pub const fn value(&self) -> u16 {
        self.w_value
    }

    /// Gets the index.
    pub const fn index(&self) -> u16 {
        self.w_index
    }

    /// Gets the length.
    pub const fn length(&self) -> u16 {
        self.w_length
    }

    /// Gets the timeout.
    pub const fn timeout(&self) -> u32 {
        self.timeout
    }

    /// Gets a reference to the data buffer.
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl From<&UsbfsCtrlTransferFfi> for UsbfsCtrlTransfer {
    fn from(val: &UsbfsCtrlTransferFfi) -> Self {
        Self {
            bm_request_type: val.bm_request_type,
            b_request: val.b_request,
            w_value: val.w_value,
            w_index: val.w_index,
            w_length: val.w_length,
            timeout: val.timeout,
            data: if val.data.is_null() {
                Vec::new()
            } else {
                unsafe { std::slice::from_raw_parts(val.data as *mut u8, val.w_length as usize).into() }
            },
        }
    }
}

impl From<UsbfsCtrlTransferFfi> for UsbfsCtrlTransfer {
    fn from(val: UsbfsCtrlTransferFfi) -> Self {
        (&val).into()
    }
}
