use std::fmt;

/// Represents USBFS stream information.
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UsbfsStreams {
    num_streams: u32,
    eps: Vec<u8>,
}

impl UsbfsStreams {
    /// Creates a new [UsbfsStreams].
    pub const fn new() -> Self {
        Self {
            num_streams: 0,
            eps: Vec::new(),
        }
    }

    /// Gets the number of streams.
    pub const fn num_streams(&self) -> u32 {
        self.num_streams
    }

    /// Sets the number of streams.
    pub fn set_num_streams(&mut self, num_streams: u32) {
        self.num_streams = num_streams;
    }

    /// Builder function that sets the number of streams.
    pub fn with_num_streams(mut self, num_streams: u32) -> Self {
        self.set_num_streams(num_streams);
        self
    }

    /// Gets a reference to the list of endpoints.
    pub fn eps(&self) -> &[u8] {
        self.eps.as_ref()
    }

    /// Sets the list of endpoints.
    pub fn set_eps<E: IntoIterator<Item = u8>>(&mut self, eps: E) {
        self.eps = eps.into_iter().collect();
    }

    /// Builder function that sets the list of endpoints.
    pub fn with_eps<E: IntoIterator<Item = u8>>(mut self, eps: E) -> Self {
        self.set_eps(eps);
        self
    }
}

impl From<&UsbfsStreamsFfi> for UsbfsStreams {
    fn from(val: &UsbfsStreamsFfi) -> Self {
        Self {
            num_streams: val.num_streams,
            eps: if val.eps.is_null() {
                Vec::new()
            } else {
                // SAFETY: memory returned by the kernel should be valid, if non-null.
                unsafe { std::slice::from_raw_parts(val.eps, val.num_eps as usize) }.into()
            },
        }
    }
}

impl From<UsbfsStreamsFfi> for UsbfsStreams {
    fn from(val: UsbfsStreamsFfi) -> Self {
        (&val).into()
    }
}

impl fmt::Display for UsbfsStreams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        write!(f, r#""num_streams": {}, "#, self.num_streams)?;
        write!(f, r#""eps": ["#)?;
        for (i, ep) in self.eps.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{ep}")?;
        }
        write!(f, "]}}")
    }
}

/// Represents USBFS stream information for `ioctl` FFI.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct UsbfsStreamsFfi {
    num_streams: u32,
    num_eps: u32,
    eps: *mut u8,
}

impl UsbfsStreamsFfi {
    /// Creates a new [UsbfsStreamsFfi].
    pub fn new() -> Self {
        Self {
            num_streams: 0,
            num_eps: 0,
            eps: std::ptr::null_mut(),
        }
    }
}

impl From<&mut UsbfsStreams> for UsbfsStreamsFfi {
    fn from(val: &mut UsbfsStreams) -> Self {
        Self {
            num_streams: val.num_streams,
            num_eps: val.eps.len() as u32,
            eps: if val.eps.is_empty() {
                std::ptr::null_mut()
            } else {
                val.eps.as_mut_ptr()
            },
        }
    }
}

impl Default for UsbfsStreamsFfi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usbfs_streams() {
        let exp_num_streams = 1;
        let exp_num_eps = 1;
        let exp_eps = [1u8];

        let exp_streams = UsbfsStreams::new()
            .with_num_streams(exp_num_streams)
            .with_eps(exp_eps);

        let mut null_streams = UsbfsStreams::new();

        assert_eq!(exp_streams.num_streams(), exp_num_streams);
        assert_eq!(exp_streams.eps(), exp_eps.as_ref());
        assert_eq!(exp_streams.eps().len(), exp_num_eps);

        null_streams.set_num_streams(exp_num_streams);
        assert_eq!(null_streams.num_streams(), exp_num_streams);

        null_streams.set_eps(exp_eps);
        assert_eq!(null_streams.eps(), exp_eps.as_ref());

        assert_eq!(null_streams, exp_streams);
    }
}
