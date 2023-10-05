use std::{ffi::c_void, fmt};

use super::UsbfsIsoPacketDesc;

/// Convenience alias for types used as a `usercontext` argument in [`Urb`].
///
/// The `usercontext` is an argument to USB callback completion functions.
pub trait UrbUserContext {
    fn as_raw_ptr(&mut self) -> *mut c_void;
}

impl UrbUserContext for () {
    fn as_raw_ptr(&mut self) -> *mut c_void {
        self as *mut () as *mut _
    }
}

impl fmt::Display for dyn UrbUserContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl fmt::Debug for dyn UrbUserContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

/// Represents USBFS transfer information.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TransferInfo(u32);

impl TransferInfo {
    /// Creates a new [TransferInfo].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Creates a new [TransferInfo] for an Isochronous transfer.
    pub const fn create_isoc(number_of_packets: i32) -> Self {
        Self(number_of_packets as u32)
    }

    /// Creates a new [TransferInfo] for a Bulk transfer.
    pub const fn create_bulk(stream_id: u32) -> Self {
        Self(stream_id)
    }

    /// Gets the number of packets in an Isochronous transfer.
    pub const fn number_of_packets(&self) -> i32 {
        self.0 as i32
    }

    /// Gets the stream ID of a Bulk transfer.
    pub const fn stream_id(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for TransferInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Represents a URB record on Linux.
#[repr(C)]
#[derive(Debug)]
pub struct Urb {
    urb_type: u8,
    endpoint: u8,
    status: i32,
    flags: u32,
    buffer: Vec<u8>,
    buffer_length: usize,
    actual_length: usize,
    start_frame: i32,
    info: TransferInfo,
    error_count: i32,
    signr: u32,
    usercontext: Box<dyn UrbUserContext>,
    iso_frame_desc: Vec<UsbfsIsoPacketDesc>,
}

impl PartialEq for Urb {
    fn eq(&self, rhs: &Self) -> bool {
        self.urb_type == rhs.urb_type
            && self.endpoint == rhs.endpoint
            && self.status == rhs.status
            && self.flags == rhs.flags
            && self.buffer == rhs.buffer
            && self.buffer_length == rhs.buffer_length
            && self.actual_length == rhs.actual_length
            && self.info == rhs.info
            && self.error_count == rhs.error_count
            && self.signr == rhs.signr
            && self.iso_frame_desc == rhs.iso_frame_desc
    }
}

impl Urb {
    /// Creates a new [Urb].
    pub fn new() -> Self {
        Self {
            urb_type: 0,
            endpoint: 0,
            status: 0,
            flags: 0,
            buffer: Vec::new(),
            buffer_length: 0,
            actual_length: 0,
            start_frame: 0,
            info: TransferInfo::new(),
            error_count: 0,
            signr: 0,
            usercontext: Box::new(()),
            iso_frame_desc: Vec::new(),
        }
    }

    /// Gets the URB type.
    pub const fn urb_type(&self) -> u8 {
        self.urb_type
    }

    /// Sets the URB type.
    pub fn set_urb_type(&mut self, urb_type: u8) {
        self.urb_type = urb_type;
    }

    /// Builder function that sets the URB type.
    pub fn with_urb_type(mut self, urb_type: u8) -> Self {
        self.set_urb_type(urb_type);
        self
    }

    /// Gets the URB endpoint.
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    /// Sets the URB endpoint.
    pub fn set_endpoint(&mut self, endpoint: u8) {
        self.endpoint = endpoint;
    }

    /// Builder function that sets the URB endpoint.
    pub fn with_endpoint(mut self, endpoint: u8) -> Self {
        self.set_endpoint(endpoint);
        self
    }

    /// Gets the URB status.
    pub const fn status(&self) -> i32 {
        self.status
    }

    /// Sets the URB status.
    pub fn set_status(&mut self, status: i32) {
        self.status = status;
    }

    /// Builder function that sets the URB status.
    pub fn with_status(mut self, status: i32) -> Self {
        self.set_status(status);
        self
    }

    /// Gets the URB flags.
    pub const fn flags(&self) -> u32 {
        self.flags
    }

    /// Sets the URB flags.
    pub fn set_flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    /// Builder function that sets the URB flags.
    pub fn with_flags(mut self, flags: u32) -> Self {
        self.set_flags(flags);
        self
    }

    /// Gets the URB buffer.
    pub fn buffer(&self) -> &[u8] {
        self.buffer.as_ref()
    }

    /// Sets the URB buffer.
    pub fn set_buffer<B: IntoIterator<Item = u8>>(&mut self, buffer: B) {
        self.buffer = buffer.into_iter().collect();
    }

    /// Builder function that sets the URB buffer.
    pub fn with_buffer<B: IntoIterator<Item = u8>>(mut self, buffer: B) -> Self {
        self.set_buffer(buffer);
        self
    }

    /// Gets the URB actual length.
    pub const fn actual_length(&self) -> usize {
        self.actual_length
    }

    /// Sets the URB actual length.
    pub fn set_actual_length(&mut self, actual_length: usize) {
        self.actual_length = actual_length;
    }

    /// Builder function that sets the URB actual length.
    pub fn with_actual_length(mut self, actual_length: usize) -> Self {
        self.set_actual_length(actual_length);
        self
    }

    /// Gets the URB start frame.
    pub const fn start_frame(&self) -> i32 {
        self.start_frame
    }

    /// Sets the URB start frame.
    pub fn set_start_frame(&mut self, start_frame: i32) {
        self.start_frame = start_frame;
    }

    /// Builder function that sets the URB start frame.
    pub fn with_start_frame(mut self, start_frame: i32) -> Self {
        self.set_start_frame(start_frame);
        self
    }

    /// Gets the URB info.
    pub const fn info(&self) -> &TransferInfo {
        &self.info
    }

    /// Sets the URB info.
    pub fn set_info(&mut self, info: TransferInfo) {
        self.info = info;
    }

    /// Builder function that sets the URB info.
    pub fn with_info(mut self, info: TransferInfo) -> Self {
        self.set_info(info);
        self
    }

    /// Gets the URB error count.
    pub const fn error_count(&self) -> i32 {
        self.error_count
    }

    /// Sets the URB error count.
    pub fn set_error_count(&mut self, error_count: i32) {
        self.error_count = error_count;
    }

    /// Builder function that sets the URB error count.
    pub fn with_error_count(mut self, error_count: i32) -> Self {
        self.set_error_count(error_count);
        self
    }

    /// Gets the URB signr.
    pub const fn signr(&self) -> u32 {
        self.signr
    }

    /// Sets the URB signr.
    pub fn set_signr(&mut self, signr: u32) {
        self.signr = signr;
    }

    /// Builder function that sets the URB signr.
    pub fn with_signr(mut self, signr: u32) -> Self {
        self.set_signr(signr);
        self
    }

    /// Gets the URB usercontext.
    pub fn usercontext(&self) -> &dyn UrbUserContext {
        self.usercontext.as_ref()
    }

    /// Sets the URB usercontext.
    pub fn set_usercontext(&mut self, usercontext: Box<dyn UrbUserContext>) {
        self.usercontext = usercontext;
    }

    /// Builder function that sets the URB usercontext.
    pub fn with_usercontext(mut self, usercontext: Box<dyn UrbUserContext>) -> Self {
        self.set_usercontext(usercontext);
        self
    }

    /// Gets the URB [UsbfsIsoPacketDesc] list.
    pub fn iso_frame_desc(&self) -> &[UsbfsIsoPacketDesc] {
        self.iso_frame_desc.as_ref()
    }

    /// Sets the URB iso_frame_desc.
    pub fn set_iso_frame_desc<B: IntoIterator<Item = UsbfsIsoPacketDesc>>(&mut self, iso_frame_desc: B) {
        self.iso_frame_desc = iso_frame_desc.into_iter().collect();
    }

    /// Builder function that sets the URB iso_frame_desc.
    pub fn with_iso_frame_desc<B: IntoIterator<Item = UsbfsIsoPacketDesc>>(mut self, iso_frame_desc: B) -> Self {
        self.set_iso_frame_desc(iso_frame_desc);
        self
    }
}

impl fmt::Display for Urb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""type": {}, "#, self.urb_type)?;
        write!(f, r#""endpoint": {}, "#, self.endpoint)?;
        write!(f, r#""status": {}, "#, self.status)?;
        write!(f, r#""flags": {}, "#, self.flags)?;

        write!(f, r#""buffer: ["#)?;
        for (i, b) in self.buffer.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{b}")?;
        }
        write!(f, "], ")?;

        write!(f, r#""buffer_length": {}, "#, self.buffer_length)?;
        write!(f, r#""actual_length": {}, "#, self.actual_length)?;
        write!(f, r#""start_frame": {}, "#, self.start_frame)?;

        if self.iso_frame_desc.is_empty() {
            write!(f, r#""stream_id": {}, "#, self.info.stream_id())?;
        } else {
            write!(f, r#""number_of_packets": {}, "#, self.info.number_of_packets())?;
        }

        write!(f, r#""error_count": {}, "#, self.error_count)?;
        write!(f, r#""signr": {}, "#, self.signr)?;

        write!(f, r#""iso_frame_desc": ["#)?;
        for (i, frame) in self.iso_frame_desc.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{frame}")?;
        }
        write!(f, "]}}")
    }
}

impl Default for Urb {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub union TransferInfoFfi {
    number_of_packets: i32,
    stream_id: u32,
}

impl TransferInfoFfi {
    /// Creates a new [TransferInfoFfi].
    pub const fn new() -> Self {
        Self { number_of_packets: 0 }
    }

    /// Creates a new [TransferInfoFfi] for isochronous transfers from the provided parameter.
    pub fn new_isoc(number_of_packets: i32) -> Self {
        Self { number_of_packets }
    }

    /// Creates a new [TransferInfoFfi] for bulk transfers from the provided parameter.
    pub fn new_bulk(stream_id: u32) -> Self {
        Self { stream_id }
    }
}

impl From<&TransferInfo> for TransferInfoFfi {
    fn from(val: &TransferInfo) -> Self {
        let number_of_packets = val.number_of_packets();
        let stream_id = val.stream_id();

        if number_of_packets != 0 {
            Self::new_isoc(number_of_packets)
        } else if stream_id != 0 {
            Self::new_bulk(stream_id) 
        } else {
            Self::new()
        }
    }
}

impl From<TransferInfo> for TransferInfoFfi {
    fn from(val: TransferInfo) -> Self {
        (&val).into()
    }
}

impl Default for TransferInfoFfi {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TransferInfoFfi {
    fn eq(&self, rhs: &Self) -> bool {
        unsafe {
            match (self, rhs) {
                (Self { number_of_packets: nl }, Self { number_of_packets: nr }) if nl == nr => true,
                (Self { stream_id: lid }, Self { stream_id: rid }) if lid == rid => true,
                _ => false,
            }
        }
    }
}

/// Represents a URB record on Linux passed to an `ioctl` FFI.
#[repr(C)]
#[derive(PartialEq)]
pub struct UrbFfi {
    urb_type: u8,
    endpoint: u8,
    status: i32,
    flags: u32,
    buffer: *mut c_void,
    buffer_length: i32,
    actual_length: i32,
    start_frame: i32,
    info: TransferInfoFfi, 
    error_count: i32,
    signr: u32,
    usercontext: *mut c_void,
    iso_frame_desc: *mut UsbfsIsoPacketDesc,
}

impl UrbFfi {
    /// Creates a new [UrbFfi].
    pub const fn new() -> Self {
        Self {
            urb_type: 0,
            endpoint: 0,
            status: 0,
            flags: 0,
            buffer: std::ptr::null_mut(),
            buffer_length: 0,
            actual_length: 0,
            start_frame: 0,
            info: TransferInfoFfi::new(), 
            error_count: 0,
            signr: 0,
            usercontext: std::ptr::null_mut(),
            iso_frame_desc: std::ptr::null_mut(),
        }
    }
}

impl From<&mut Urb> for UrbFfi {
    fn from(val: &mut Urb) -> Self {
        let info = if val.iso_frame_desc.is_empty() {
            val.info.into()
        } else {
            let len = val.iso_frame_desc.len();
            // check that we don't overflow the length
            if len > i32::MAX as usize {
                TransferInfoFfi::new_isoc(i32::MAX)
            } else {
                TransferInfoFfi::new_isoc(len as i32)
            }
        };

        Self {
            urb_type: val.urb_type,
            endpoint: val.endpoint,
            status: val.status,
            flags: val.flags,
            buffer: val.buffer.as_mut_ptr() as *mut _,
            buffer_length: val.buffer.len() as i32,
            actual_length: val.actual_length as i32,
            start_frame: val.start_frame,
            info, 
            error_count: val.error_count,
            signr: val.signr,
            usercontext: val.usercontext.as_raw_ptr(),
            iso_frame_desc: if val.iso_frame_desc.is_empty() {
                std::ptr::null_mut()
            } else {
                val.iso_frame_desc.as_mut_ptr() as *mut _
            },
        }
    }
}

impl Default for UrbFfi {
    fn default() -> Self {
        Self::new()
    }
}
