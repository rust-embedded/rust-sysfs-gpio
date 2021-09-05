// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.
//
// Portions of this implementation are based on work by Nat Pryce:
// https://github.com/npryce/rusty-pi/blob/master/src/pi/gpio.rs

//! GPIO access under Linux using the GPIO sysfs interface
//!
//! The methods exposed by this library are centered around
//! the `Pin` struct and map pretty directly the API exposed
//! by the kernel in syfs <https://www.kernel.org/doc/Documentation/gpio/sysfs.txt>.
//!
//! # Examples
//!
//! Typical usage for systems where one wants to ensure that
//! the pins in use are unexported upon completion looks like
//! the following:
//!
//! ```no_run
//! extern crate sysfs_gpio;
//!
//! use sysfs_gpio::{Direction, Pin};
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! fn main() {
//!     let my_led = Pin::new(127); // number depends on chip, etc.
//!     my_led.with_exported(|| {
//!         my_led.set_direction(Direction::Out).unwrap();
//!         loop {
//!             my_led.set_value(0).unwrap();
//!             sleep(Duration::from_millis(200));
//!             my_led.set_value(1).unwrap();
//!             sleep(Duration::from_millis(200));
//!         }
//!     }).unwrap();
//! }
//! ```

#![cfg_attr(feature = "async-tokio", allow(deprecated))]

#[cfg(feature = "async-tokio")]
extern crate futures;
#[cfg(feature = "mio-evented")]
extern crate mio;
#[cfg(not(target_os = "wasi"))]
extern crate nix;
#[cfg(feature = "async-tokio")]
extern crate tokio;

use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
#[cfg(any(target_os = "linux", target_os = "android", feature = "async-tokio"))]
use std::io::SeekFrom;
#[cfg(not(target_os = "wasi"))]
use std::os::unix::prelude::*;
use std::path::Path;

#[cfg(feature = "async-tokio")]
use futures::{Async, Poll, Stream};
#[cfg(feature = "mio-evented")]
use mio::unix::EventedFd;
#[cfg(feature = "mio-evented")]
use mio::Evented;
#[cfg(any(target_os = "linux", target_os = "android"))]
use nix::sys::epoll::*;
#[cfg(not(target_os = "wasi"))]
use nix::unistd::close;
#[cfg(feature = "async-tokio")]
use tokio::reactor::{Handle, PollEvented};

pub use error::Error;

mod error;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pin {
    pin_num: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
    High,
    Low,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    NoInterrupt,
    RisingEdge,
    FallingEdge,
    BothEdges,
}

#[macro_export]
macro_rules! try_unexport {
    ($gpio:ident, $e:expr) => {
        match $e {
            Ok(res) => res,
            Err(e) => {
                $gpio.unexport()?;
                return Err(e);
            }
        }
    };
}

pub type Result<T> = ::std::result::Result<T, error::Error>;

/// Flush up to max bytes from the provided files input buffer
///
/// Typically, one would just use seek() for this sort of thing,
/// but for certain files (e.g. in sysfs), you need to actually
/// read it.
#[cfg(any(target_os = "linux", target_os = "android"))]
fn flush_input_from_file(dev_file: &mut File, max: usize) -> io::Result<usize> {
    let mut s = String::with_capacity(max);
    dev_file.read_to_string(&mut s)
}

/// Get the pin value from the provided file
#[cfg(any(target_os = "linux", target_os = "android", feature = "async-tokio"))]
fn get_value_from_file(dev_file: &mut File) -> Result<u8> {
    let mut s = String::with_capacity(10);
    dev_file.seek(SeekFrom::Start(0))?;
    dev_file.read_to_string(&mut s)?;
    match s[..1].parse::<u8>() {
        Ok(n) => Ok(n),
        Err(_) => Err(Error::Unexpected(format!(
            "Unexpected value file contents: {:?}",
            s
        ))),
    }
}

impl Pin {
    /// Write all of the provided contents to the specified devFile
    fn write_to_device_file(&self, dev_file_name: &str, value: &str) -> io::Result<()> {
        let gpio_path = format!("/sys/class/gpio/gpio{}/{}", self.pin_num, dev_file_name);
        let mut dev_file = File::create(&gpio_path)?;
        dev_file.write_all(value.as_bytes())?;
        Ok(())
    }

    fn read_from_device_file(&self, dev_file_name: &str) -> io::Result<String> {
        let gpio_path = format!("/sys/class/gpio/gpio{}/{}", self.pin_num, dev_file_name);
        let mut dev_file = File::open(&gpio_path)?;
        let mut s = String::new();
        dev_file.read_to_string(&mut s)?;
        Ok(s)
    }

    /// Create a new Pin with the provided `pin_num`
    ///
    /// This function does not export the provided pin_num.
    pub fn new(pin_num: u64) -> Pin {
        Pin { pin_num }
    }

    /// Create a new Pin with the provided path
    ///
    /// This form is useful when there are other scripts which may
    /// have already exported the GPIO and created a symlink with a
    /// nice name that you already have reference to.  Otherwise, it
    /// is generally preferrable to use `new` directly.
    ///
    /// The provided path must be either the already exported
    /// directory for a GPIO or a symlink to one.  If the directory
    /// does not look sufficiently like this (i.e. does not resolve to
    /// a path starting with /sys/class/gpioXXX), then this function
    /// will return an error.
    pub fn from_path<T: AsRef<Path>>(path: T) -> Result<Pin> {
        // Resolve all symbolic links in the provided path
        let pb = fs::canonicalize(path.as_ref())?;

        // determine if this is valid and figure out the pin_num
        if !fs::metadata(&pb)?.is_dir() {
            return Err(Error::Unexpected(
                "Provided path not a directory or symlink to \
                                          a directory"
                    .to_owned(),
            ));
        }
        let num = Pin::extract_pin_from_path(&pb)?;
        Ok(Pin::new(num))
    }

    /// Extract pin number from paths like /sys/class/gpio/gpioXXX
    fn extract_pin_from_path<P: AsRef<Path>>(path: P) -> Result<u64> {
        path.as_ref()
            .file_name()
            .and_then(|filename| filename.to_str())
            .and_then(|filename_str| filename_str.trim_start_matches("gpio").parse::<u64>().ok())
            .ok_or_else(|| Error::InvalidPath(format!("{:?}", path.as_ref())))
    }

    /// Get the pin number
    pub fn get_pin_num(&self) -> u64 {
        self.pin_num
    }

    /// Run a closure with the GPIO exported
    ///
    /// Prior to the provided closure being executed, the GPIO
    /// will be exported.  After the closure execution is complete,
    /// the GPIO will be unexported.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use sysfs_gpio::{Pin, Direction};
    ///
    /// let gpio = Pin::new(24);
    /// let res = gpio.with_exported(|| {
    ///     println!("At this point, the Pin is exported");
    ///     gpio.set_direction(Direction::Low)?;
    ///     gpio.set_value(1)?;
    ///     // ...
    ///     Ok(())
    /// });
    /// ```
    #[inline]
    pub fn with_exported<F: FnOnce() -> Result<()>>(&self, closure: F) -> Result<()> {
        self.export()?;
        match closure() {
            Ok(()) => {
                self.unexport()?;
                Ok(())
            }
            Err(err) => {
                self.unexport()?;
                Err(err)
            }
        }
    }

    /// Determines whether the GPIO is exported
    ///
    /// This function will error out if the kernel does not support the GPIO
    /// sysfs interface (i.e. `/sys/class/gpio` does not exist).
    pub fn is_exported(&self) -> bool {
        fs::metadata(&format!("/sys/class/gpio/gpio{}", self.pin_num)).is_ok()
    }

    /// Export the GPIO
    ///
    /// This is equivalent to `echo N > /sys/class/gpio/export` with
    /// the exception that the case where the GPIO is already exported
    /// is not an error.
    ///
    /// # Errors
    ///
    /// The main cases in which this function will fail and return an
    /// error are the following:
    /// 1. The system does not support the GPIO sysfs interface
    /// 2. The requested GPIO is out of range and cannot be exported
    /// 3. The requested GPIO is in use by the kernel and cannot
    ///    be exported by use in userspace
    ///
    /// # Example
    /// ```no_run
    /// use sysfs_gpio::Pin;
    ///
    /// let gpio = Pin::new(24);
    /// match gpio.export() {
    ///     Ok(()) => println!("Gpio {} exported!", gpio.get_pin()),
    ///     Err(err) => println!("Gpio {} could not be exported: {}", gpio.get_pin(), err),
    /// }
    /// ```
    pub fn export(&self) -> Result<()> {
        if fs::metadata(&format!("/sys/class/gpio/gpio{}", self.pin_num)).is_err() {
            let mut export_file = File::create("/sys/class/gpio/export")?;
            export_file.write_all(format!("{}", self.pin_num).as_bytes())?;
        }
        Ok(())
    }

    /// Unexport the GPIO
    ///
    /// This function will unexport the provided by from syfs if
    /// it is currently exported.  If the pin is not currently
    /// exported, it will return without error.  That is, whenever
    /// this function returns Ok, the GPIO is not exported.
    pub fn unexport(&self) -> Result<()> {
        if fs::metadata(&format!("/sys/class/gpio/gpio{}", self.pin_num)).is_ok() {
            let mut unexport_file = File::create("/sys/class/gpio/unexport")?;
            unexport_file.write_all(format!("{}", self.pin_num).as_bytes())?;
        }
        Ok(())
    }

    /// Get the pin number for the Pin
    pub fn get_pin(&self) -> u64 {
        self.pin_num
    }

    /// Get the direction of the Pin
    pub fn get_direction(&self) -> Result<Direction> {
        match self.read_from_device_file("direction") {
            Ok(s) => match s.trim() {
                "in" => Ok(Direction::In),
                "out" => Ok(Direction::Out),
                "high" => Ok(Direction::High),
                "low" => Ok(Direction::Low),
                other => Err(Error::Unexpected(format!(
                    "direction file contents {}",
                    other
                ))),
            },
            Err(e) => Err(::std::convert::From::from(e)),
        }
    }

    /// Set this GPIO as either an input or an output
    ///
    /// The basic values allowed here are `Direction::In` and
    /// `Direction::Out` which set the Pin as either an input
    /// or output respectively.  In addition to those, two
    /// additional settings of `Direction::High` and
    /// `Direction::Low`.  These both set the Pin as an output
    /// but do so with an initial value of high or low respectively.
    /// This allows for glitch-free operation.
    ///
    /// Note that this entry may not exist if the kernel does
    /// not support changing the direction of a pin in userspace.  If
    /// this is the case, you will get an error.
    pub fn set_direction(&self, dir: Direction) -> Result<()> {
        self.write_to_device_file(
            "direction",
            match dir {
                Direction::In => "in",
                Direction::Out => "out",
                Direction::High => "high",
                Direction::Low => "low",
            },
        )?;

        Ok(())
    }

    /// Get the value of the Pin (0 or 1)
    ///
    /// If successful, 1 will be returned if the pin is high
    /// and 0 will be returned if the pin is low (this may or may
    /// not match the signal level of the actual signal depending
    /// on the GPIO "active_low" entry).
    pub fn get_value(&self) -> Result<u8> {
        match self.read_from_device_file("value") {
            Ok(s) => match s.trim() {
                "1" => Ok(1),
                "0" => Ok(0),
                other => Err(Error::Unexpected(format!("value file contents {}", other))),
            },
            Err(e) => Err(::std::convert::From::from(e)),
        }
    }

    /// Set the value of the Pin
    ///
    /// This will set the value of the pin either high or low.
    /// A 0 value will set the pin low and any other value will
    /// set the pin high (1 is typical).
    pub fn set_value(&self, value: u8) -> Result<()> {
        self.write_to_device_file(
            "value",
            match value {
                0 => "0",
                _ => "1",
            },
        )?;

        Ok(())
    }

    /// Get the currently configured edge for this pin
    ///
    /// This value will only be present if the Pin allows
    /// for interrupts.
    pub fn get_edge(&self) -> Result<Edge> {
        match self.read_from_device_file("edge") {
            Ok(s) => match s.trim() {
                "none" => Ok(Edge::NoInterrupt),
                "rising" => Ok(Edge::RisingEdge),
                "falling" => Ok(Edge::FallingEdge),
                "both" => Ok(Edge::BothEdges),
                other => Err(Error::Unexpected(format!(
                    "Unexpected file contents {}",
                    other
                ))),
            },
            Err(e) => Err(::std::convert::From::from(e)),
        }
    }

    /// Set the edge on which this GPIO will trigger when polled
    ///
    /// The configured edge determines what changes to the Pin will
    /// result in `poll()` returning.  This call will return an Error
    /// if the pin does not allow interrupts.
    pub fn set_edge(&self, edge: Edge) -> Result<()> {
        self.write_to_device_file(
            "edge",
            match edge {
                Edge::NoInterrupt => "none",
                Edge::RisingEdge => "rising",
                Edge::FallingEdge => "falling",
                Edge::BothEdges => "both",
            },
        )?;

        Ok(())
    }

    /// Get polarity of the Pin (`true` is active low)
    pub fn get_active_low(&self) -> Result<bool> {
        match self.read_from_device_file("active_low") {
            Ok(s) => match s.trim() {
                "1" => Ok(true),
                "0" => Ok(false),
                other => Err(Error::Unexpected(format!(
                    "active_low file contents {}",
                    other
                ))),
            },
            Err(e) => Err(::std::convert::From::from(e)),
        }
    }

    /// Set the polarity of the Pin (`true` is active low)
    ///
    /// This will affect "rising" and "falling" edge triggered
    /// configuration.
    pub fn set_active_low(&self, active_low: bool) -> Result<()> {
        self.write_to_device_file(
            "active_low",
            match active_low {
                true => "1",
                false => "0",
            },
        )?;

        Ok(())
    }

    /// Get a PinPoller object for this pin
    ///
    /// This pin poller object will register an interrupt with the
    /// kernel and allow you to poll() on it and receive notifications
    /// that an interrupt has occured with minimal delay.
    #[cfg(not(target_os = "wasi"))]
    pub fn get_poller(&self) -> Result<PinPoller> {
        PinPoller::new(self.pin_num)
    }

    /// Get an AsyncPinPoller object for this pin
    ///
    /// The async pin poller object can be used with the `mio` crate. You should probably call
    /// `set_edge()` before using this.
    ///
    /// This method is only available when the `mio-evented` crate feature is enabled.
    #[cfg(feature = "mio-evented")]
    pub fn get_async_poller(&self) -> Result<AsyncPinPoller> {
        AsyncPinPoller::new(self.pin_num)
    }

    /// Get a Stream of pin interrupts for this pin
    ///
    /// The PinStream object can be used with the `tokio` crate. You should probably call
    /// `set_edge()` before using this.
    ///
    /// This method is only available when the `async-tokio` crate feature is enabled.
    #[cfg(feature = "async-tokio")]
    pub fn get_stream_with_handle(&self, handle: &Handle) -> Result<PinStream> {
        PinStream::init_with_handle(*self, handle)
    }

    /// Get a Stream of pin interrupts for this pin
    ///
    /// The PinStream object can be used with the `tokio` crate. You should probably call
    /// `set_edge()` before using this.
    ///
    /// This method is only available when the `async-tokio` crate feature is enabled.
    #[cfg(feature = "async-tokio")]
    pub fn get_stream(&self) -> Result<PinStream> {
        PinStream::init(*self)
    }

    /// Get a Stream of pin values for this pin
    ///
    /// The PinStream object can be used with the `tokio` crate. You should probably call
    /// `set_edge(Edge::BothEdges)` before using this.
    ///
    /// Note that the values produced are the value of the pin as soon as we get to handling the
    /// interrupt in userspace.  Each time this stream produces a value, a change has occurred, but
    /// it could end up producing the same value multiple times if the value has changed back
    /// between when the interrupt occurred and when the value was read.
    ///
    /// This method is only available when the `async-tokio` crate feature is enabled.
    #[cfg(feature = "async-tokio")]
    pub fn get_value_stream_with_handle(&self, handle: &Handle) -> Result<PinValueStream> {
        Ok(PinValueStream(PinStream::init_with_handle(*self, handle)?))
    }

    /// Get a Stream of pin values for this pin
    ///
    /// The PinStream object can be used with the `tokio` crate. You should probably call
    /// `set_edge(Edge::BothEdges)` before using this.
    ///
    /// Note that the values produced are the value of the pin as soon as we get to handling the
    /// interrupt in userspace.  Each time this stream produces a value, a change has occurred, but
    /// it could end up producing the same value multiple times if the value has changed back
    /// between when the interrupt occurred and when the value was read.
    ///
    /// This method is only available when the `async-tokio` crate feature is enabled.
    #[cfg(feature = "async-tokio")]
    pub fn get_value_stream(&self) -> Result<PinValueStream> {
        Ok(PinValueStream(PinStream::init(*self)?))
    }
}

#[test]
fn extract_pin_fom_path_test() {
    let tok1 = Pin::extract_pin_from_path(&"/sys/class/gpio/gpio951");
    assert_eq!(951, tok1.unwrap());
    let tok2 = Pin::extract_pin_from_path(&"/sys/CLASS/gpio/gpio951/");
    assert_eq!(951, tok2.unwrap());
    let tok3 = Pin::extract_pin_from_path(&"../../devices/soc0/gpiochip3/gpio/gpio124");
    assert_eq!(124, tok3.unwrap());
    let err1 = Pin::extract_pin_from_path(&"/sys/CLASS/gpio/gpio");
    assert_eq!(true, err1.is_err());
    let err2 = Pin::extract_pin_from_path(&"/sys/class/gpio/gpioSDS");
    assert_eq!(true, err2.is_err());
}
#[cfg(not(target_os = "wasi"))]
#[derive(Debug)]
pub struct PinPoller {
    pin_num: u64,
    epoll_fd: RawFd,
    devfile: File,
}
#[cfg(not(target_os = "wasi"))]
impl PinPoller {
    /// Get the pin associated with this PinPoller
    ///
    /// Note that this will be a new Pin object with the
    /// proper pin number.
    pub fn get_pin(&self) -> Pin {
        Pin::new(self.pin_num)
    }

    /// Create a new PinPoller for the provided pin number
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn new(pin_num: u64) -> Result<PinPoller> {
        let devfile: File = File::open(&format!("/sys/class/gpio/gpio{}/value", pin_num))?;
        let devfile_fd = devfile.as_raw_fd();
        let epoll_fd = epoll_create()?;
        let mut event = EpollEvent::new(EpollFlags::EPOLLPRI | EpollFlags::EPOLLET, 0u64);

        match epoll_ctl(epoll_fd, EpollOp::EpollCtlAdd, devfile_fd, &mut event) {
            Ok(_) => Ok(PinPoller {
                pin_num,
                devfile,
                epoll_fd,
            }),
            Err(err) => {
                let _ = close(epoll_fd); // cleanup
                Err(::std::convert::From::from(err))
            }
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    pub fn new(_pin_num: u64) -> Result<PinPoller> {
        Err(Error::Unsupported("PinPoller".into()))
    }

    /// Block until an interrupt occurs
    ///
    /// This call will block until an interrupt occurs.  The types
    /// of interrupts which may result in this call returning
    /// may be configured by calling `set_edge()` prior to
    /// making this call.  This call makes use of epoll under the
    /// covers.  To poll on multiple GPIOs or other event sources,
    /// poll asynchronously using the integration with either `mio`
    /// or `tokio`.
    ///
    /// This function will return Some(value) of the pin if a change is
    /// detected or None if a timeout occurs.  Note that the value provided
    /// is the value of the pin as soon as we get to handling the interrupt
    /// in userspace.  Each time this function returns with a value, a change
    /// has occurred, but you could end up reading the same value multiple
    /// times as the value has changed back between when the interrupt
    /// occurred and the current time.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    pub fn poll(&mut self, timeout_ms: isize) -> Result<Option<u8>> {
        flush_input_from_file(&mut self.devfile, 255)?;
        let dummy_event = EpollEvent::new(EpollFlags::EPOLLPRI | EpollFlags::EPOLLET, 0u64);
        let mut events: [EpollEvent; 1] = [dummy_event];
        let cnt = epoll_wait(self.epoll_fd, &mut events, timeout_ms)?;
        Ok(match cnt {
            0 => None, // timeout
            _ => Some(get_value_from_file(&mut self.devfile)?),
        })
    }

    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    pub fn poll(&mut self, _timeout_ms: isize) -> Result<Option<u8>> {
        Err(Error::Unsupported("PinPoller".into()))
    }
}

#[cfg(not(target_os = "wasi"))]
impl Drop for PinPoller {
    fn drop(&mut self) {
        // we implement drop to close the underlying epoll fd as
        // it does not implement drop itself.  This is similar to
        // how mio works
        close(self.epoll_fd).unwrap(); // panic! if close files
    }
}

#[cfg(feature = "mio-evented")]
#[derive(Debug)]
pub struct AsyncPinPoller {
    devfile: File,
}

#[cfg(feature = "mio-evented")]
impl AsyncPinPoller {
    fn new(pin_num: u64) -> Result<Self> {
        let devfile = File::open(&format!("/sys/class/gpio/gpio{}/value", pin_num))?;
        Ok(AsyncPinPoller { devfile })
    }
}

#[cfg(feature = "mio-evented")]
impl Evented for AsyncPinPoller {
    fn register(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.devfile.as_raw_fd()).register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &mio::Poll,
        token: mio::Token,
        interest: mio::Ready,
        opts: mio::PollOpt,
    ) -> io::Result<()> {
        EventedFd(&self.devfile.as_raw_fd()).reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> io::Result<()> {
        EventedFd(&self.devfile.as_raw_fd()).deregister(poll)
    }
}

#[cfg(feature = "async-tokio")]
pub struct PinStream {
    evented: PollEvented<AsyncPinPoller>,
    skipped_first_event: bool,
}

#[cfg(feature = "async-tokio")]
impl PinStream {
    pub fn init_with_handle(pin: Pin, handle: &Handle) -> Result<Self> {
        Ok(PinStream {
            evented: PollEvented::new(pin.get_async_poller()?, handle)?,
            skipped_first_event: false,
        })
    }
}

#[cfg(feature = "async-tokio")]
impl PinStream {
    pub fn init(pin: Pin) -> Result<Self> {
        Ok(PinStream {
            evented: PollEvented::new(pin.get_async_poller()?, &Handle::default())?,
            skipped_first_event: false,
        })
    }
}

#[cfg(feature = "async-tokio")]
impl Stream for PinStream {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        Ok(match self.evented.poll_read() {
            Async::Ready(()) => {
                self.evented.need_read()?;
                if self.skipped_first_event {
                    Async::Ready(Some(()))
                } else {
                    self.skipped_first_event = true;
                    Async::NotReady
                }
            }
            Async::NotReady => Async::NotReady,
        })
    }
}

#[cfg(feature = "async-tokio")]
pub struct PinValueStream(PinStream);

#[cfg(feature = "async-tokio")]
impl PinValueStream {
    #[inline]
    fn get_value(&mut self) -> Result<u8> {
        get_value_from_file(&mut self.0.evented.get_mut().devfile)
    }
}

#[cfg(feature = "async-tokio")]
impl Stream for PinValueStream {
    type Item = u8;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        match self.0.poll() {
            Ok(Async::Ready(Some(()))) => {
                let value = self.get_value()?;
                Ok(Async::Ready(Some(value)))
            }
            Ok(Async::Ready(None)) => Ok(Async::Ready(None)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}
