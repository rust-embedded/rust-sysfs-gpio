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

use std::io::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use std::fs;
use std::fs::{File};

pub struct Pin {
    pin_num : u64,
}

#[derive(Copy,Debug)]
pub enum Direction {In, Out, High, Low}

#[derive(Copy,Debug)]
pub enum Edge {NoInterrupt, RisingEdge, FallingEdge, BothEdges}

#[macro_export]
macro_rules! try_unexport {
    ($gpio:ident, $e:expr) => (match $e {
        Ok(res) => res,
        Err(e) => { try!($gpio.unexport()); return Err(e) },
    });
}

impl Pin {
    /// Write all of the provided contents to the specified devFile
    fn write_to_device_file(&self, dev_file_name: &str, value: &str) -> io::Result<()> {
        let gpio_path = format!("/sys/class/gpio/gpio{}/{}", self.pin_num, dev_file_name);
        let mut dev_file = try!(File::create(&gpio_path));
        try!(dev_file.write_all(value.as_bytes()));
        Ok(())
    }
    
    /// Create a new Pin with the provided pin_num
    ///
    /// This function does not export the provided pin_num.  If that
    /// functionality is desired, `export` should be used instead.
    pub fn new(pin_num : u64) -> Pin {
        Pin {
            pin_num: pin_num,
        }
    }

    /// Export the GPIO
    ///
    /// This is equivalent to `echo N > /sys/class/gpio/export` with
    /// the exception that the case where the GPIO is already exported
    /// is not an error.
    pub fn export(&self) -> io::Result<()> {
        match fs::metadata(&format!("/sys/class/gpio/gpio{}", self.pin_num)) {
            Ok(_) => {},
            Err(_) => {
                let mut export_file = try!(File::create("/sys/class/gpio/export"));
                try!(export_file.write_all(format!("{}", self.pin_num).as_bytes()));
            }
        };
        Ok(())
    }

    /// Unexport the GPIO
    ///
    /// This function will unexport the provided by from syfs if
    /// it is currently exported.  If the pin is not currently
    /// exported, it will return without error.  That is, whenever
    /// this function returns Ok, the GPIO is not exported.
    pub fn unexport(&self) -> io::Result<()> {
        match fs::metadata(&format!("/sys/class/gpio/{}", self.pin_num)) {
            Ok(_) => {
                let mut unexport_file = try!(File::create("/sys/class/gpio/unexport"));
                try!(unexport_file.write_all(format!("{}", self.pin_num).as_bytes()));
            },
            Err(_) => {} // not exported
        };
        Ok(())
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
    pub fn set_direction(&self, dir : Direction) -> io::Result<()> {
        self.write_to_device_file("direction", match dir {
            Direction::In => "in",
            Direction::Out => "out",
            Direction::High => "high",
            Direction::Low => "low",
        })
    }

    /// Get the value of the Pin (0 or 1)
    ///
    /// If successful, 1 will be returned if the pin is high
    /// and 0 will be returned if the pin is low (this may or may
    /// not match the signal level of the actual signal depending
    /// on the GPIO "active_low" entry).
    pub fn get_value(&self) -> io::Result<u8> {
        let mut dev_file = try!(File::open(&format!("/sys/class/gpio/gpio{}/value", self.pin_num)));
        let mut s = String::with_capacity(10);
        try!(dev_file.read_to_string(&mut s));
        match s.parse::<u8>() {
            Ok(n) => Ok(n),
            Err(_) => Err(Error::new(ErrorKind::Other, "Unexpected Error", None)),
        }
    }

    /// Set the value of the Pin
    ///
    /// This will set the value of the pin either high or low.
    /// A 0 value will set the pin low and any other value will
    /// set the pin high (1 is typical).
    pub fn set_value(&self, value : u8) -> io::Result<()> {
        let val = if value == 0 {
            "0"
        } else {
            "1"
        };
        self.write_to_device_file("value", val)
    }
}
