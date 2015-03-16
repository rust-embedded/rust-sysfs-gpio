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
use std::fs::File;

pub struct Pin {
    pin_num : u64,
}

#[derive(Copy,Debug)]
pub enum Direction {In, Out}

#[derive(Copy,Debug)]
pub enum Edge {NoInterrupt, RisingEdge, FallingEdge, BothEdges}

/// Requested that a GPIO be exported by sysfs
///
/// This is equivalent to `echo N > /sys/class/gpio/export` with
/// the exception that the case where the GPIO is already exported
/// is not an error.
pub fn export(pin_num : u64) -> io::Result<Pin> {
    let mut export_file = try!(File::create("/sys/class/gpio/export"));
    try!(export_file.write_all(format!("{}", pin_num).as_bytes()));
    Ok(Pin::new(pin_num))
}

impl Pin {
    /// Write all of the provided contents to the specified devFile
    fn write_to_device_file(&self, dev_file_name: &str, value: &str) -> io::Result<()> {
        let mut dev_file = try!(File::create(&format!("/sys/class/gpio/gpio{}/{}", self.pin_num, dev_file_name)));
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

    /// Set this GPIO as either an input or an output
    pub fn set_direction(&self, dir : Direction) -> io::Result<()> {
        self.write_to_device_file("direction", match dir {
            Direction::In => "in",
            Direction::Out => "out",
        })
    }

    /// Get the value of the GPIO (0 or 1)
    pub fn get_value(&self) -> io::Result<u8> {
        let mut dev_file = try!(File::open(&format!("/sys/class/gpio/gpio{}/value", self.pin_num)));
        let mut s = String::with_capacity(10);
        try!(dev_file.read_to_string(&mut s));
        match s.parse::<u8>() {
            Ok(n) => Ok(n),
            Err(_) => Err(Error::new(ErrorKind::Other, "Unexpected Error", None)),
        }
    }

    pub fn set_value(&self, value : u8) -> io::Result<()> {
        let val = if value == 0 {
            "0"
        } else {
            "1"
        };
        self.write_to_device_file("value", val)
    }
}
