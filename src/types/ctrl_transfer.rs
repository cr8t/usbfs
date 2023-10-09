use std::ffi::c_void;

/// Maximum length for Control data transfers
pub const MAX_CTRL_DATA: usize = u16::MAX as usize;

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

    /// Sets the request type.
    pub fn set_request_type(&mut self, req_type: u8) {
        self.bm_request_type = req_type;
    }

    /// Builder function that sets the request type.
    pub fn with_request_type(mut self, req_type: u8) -> Self {
        self.set_request_type(req_type);
        self
    }

    /// Gets the request.
    pub const fn request(&self) -> u8 {
        self.b_request
    }

    /// Sets the request.
    pub fn set_request(&mut self, req: u8) {
        self.b_request = req;
    }

    /// Builder function that sets the request.
    pub fn with_request(mut self, req: u8) -> Self {
        self.set_request(req);
        self
    }

    /// Gets the value.
    pub const fn value(&self) -> u16 {
        self.w_value
    }

    /// Sets the value.
    pub fn set_value(&mut self, value: u16) {
        self.w_value = value;
    }

    /// Builder function that sets the value.
    pub fn with_value(mut self, value: u16) -> Self {
        self.set_value(value);
        self
    }

    /// Gets the index.
    pub const fn index(&self) -> u16 {
        self.w_index
    }

    /// Sets the index.
    pub fn set_index(&mut self, index: u16) {
        self.w_index = index;
    }

    /// Builder function that sets the index.
    pub fn with_index(mut self, index: u16) -> Self {
        self.set_index(index);
        self
    }

    /// Gets the length.
    pub const fn length(&self) -> u16 {
        self.w_length
    }

    /// Gets the timeout.
    pub const fn timeout(&self) -> u32 {
        self.timeout
    }

    /// Sets the timeout.
    pub fn set_timeout(&mut self, timeout: u32) {
        self.timeout = timeout;
    }

    /// Builder function that sets the timeout.
    pub fn with_timeout(mut self, timeout: u32) -> Self {
        self.set_timeout(timeout);
        self
    }

    /// Gets a reference to the data buffer.
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    /// Sets the data buffer.
    ///
    /// **NOTE** Sets at most [`MAX_CTRL_DATA`] bytes.
    pub fn set_data<D: IntoIterator<Item = u8>>(&mut self, data: D) {
        self.data = data.into_iter().take(MAX_CTRL_DATA).collect();
        self.w_length = self.data.len() as u16;
    }

    /// Builder function that sets the data buffer.
    ///
    /// **NOTE** Sets at most [`MAX_CTRL_DATA`] bytes.
    pub fn with_data<D: IntoIterator<Item = u8>>(mut self, data: D) -> Self {
        self.set_data(data);
        self
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
                // SAFETY: converting from raw pointer data is inherently unsafe. Besides a NULL
                // check, there is no test for pointer validity. The memory handed to us from the
                // kernel is assumed contiguous, valid memory, because if it's not, many things
                // have gone wrong.
                unsafe {
                    std::slice::from_raw_parts(val.data as *mut u8, val.w_length as usize).into()
                }
            },
        }
    }
}

impl From<UsbfsCtrlTransferFfi> for UsbfsCtrlTransfer {
    fn from(val: UsbfsCtrlTransferFfi) -> Self {
        (&val).into()
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_ctrl_transfer() {
        let mut null_xfer = UsbfsCtrlTransfer::new();

        let exp_request_type = 1;
        let exp_request = 2;
        let exp_value = 3;
        let exp_index = 4;
        let exp_timeout = 5;
        let exp_data = [42u8];

        let exp_xfer = UsbfsCtrlTransfer::new()
            .with_request_type(exp_request_type)
            .with_request(exp_request)
            .with_value(exp_value)
            .with_index(exp_index)
            .with_timeout(exp_timeout)
            .with_data(exp_data);

        assert_eq!(null_xfer.request_type(), 0);
        assert_eq!(null_xfer.request(), 0);
        assert_eq!(null_xfer.value(), 0);
        assert_eq!(null_xfer.index(), 0);
        assert_eq!(null_xfer.length(), 0);
        assert_eq!(null_xfer.timeout(), 0);
        assert_eq!(null_xfer.data(), &[]);

        null_xfer.set_request_type(exp_request_type);
        assert_eq!(null_xfer.request_type(), exp_request_type);

        null_xfer.set_request(exp_request);
        assert_eq!(null_xfer.request(), exp_request);

        null_xfer.set_value(exp_value);
        assert_eq!(null_xfer.value(), exp_value);

        null_xfer.set_index(exp_index);
        assert_eq!(null_xfer.index(), exp_index);

        null_xfer.set_timeout(exp_timeout);
        assert_eq!(null_xfer.timeout(), exp_timeout);

        null_xfer.set_data(exp_data);
        assert_eq!(exp_xfer.data(), exp_data.as_ref());
        assert_eq!(exp_xfer.length(), exp_data.len() as u16);

        assert_eq!(exp_xfer.request_type(), exp_request_type);
        assert_eq!(exp_xfer.request(), exp_request);
        assert_eq!(exp_xfer.value(), exp_value);
        assert_eq!(exp_xfer.index(), exp_index);
        assert_eq!(exp_xfer.length(), exp_data.len() as u16);
        assert_eq!(exp_xfer.timeout(), exp_timeout);
        assert_eq!(exp_xfer.data(), exp_data.as_ref());

        // Check that the max data length is enforced
        assert_eq!(
            UsbfsCtrlTransfer::new()
                .with_data([0u8; MAX_CTRL_DATA + 1])
                .data(),
            [0u8; MAX_CTRL_DATA].as_ref()
        );
        assert_eq!(
            UsbfsCtrlTransfer::new()
                .with_data([0u8; MAX_CTRL_DATA + 1])
                .length(),
            MAX_CTRL_DATA as u16
        );
    }
}
