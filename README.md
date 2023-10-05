# USBFS

Pure Rust interface to the [Linux USBFS API](https://www.kernel.org/doc/html/latest/driver-api/usb/usb.html#the-usb-character-device-nodes).

Uses the [`nix`](https://crates.io/crates/nix) crate to call the `USBDEVFS` IOCTL functions.

**WARNING** This crate is very early in development. It requires a test-suite, use-case testing, and further review/development.

Pull requests and issues are very welcome :)
