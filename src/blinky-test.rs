// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(old_io)]
#![feature(std_misc)]
#![allow(deprecated)] // old_io Timer replacement not stable

extern crate sysfs_gpio;

use sysfs_gpio::core as gpio;
use std::time::Duration;
use std::old_io::Timer;
use std::io;

// export a GPIO for use.  This will not fail
// if already exported
fn blink_my_led(led : u64, duration_ms : i64, period_ms : i64) -> io::Result<()> {
    let my_led = try!(gpio::export(led));
    try!(my_led.set_direction(gpio::Direction::Out));
    let mut tmr = match Timer::new() {
        Ok(tmr) => tmr,
        Err(_) => panic!("Could not create timer!"),
    };
    let iterations = duration_ms / period_ms / 2;
    for _ in 0..iterations {
        try!(my_led.set_value(0));
        tmr.sleep(Duration::milliseconds(period_ms)); // ms
        try!(my_led.set_value(1));
        tmr.sleep(Duration::milliseconds(period_ms)); // ms
    }

    // NOTE: we do not unexport here.  Handling the
    // error case is tricky and means we cannot
    // use try!.  This is where the higher-level
    // API can work out nicely.
    return Ok(());
}

fn main() {
    match blink_my_led(66, 5000, 200) {
        Ok(()) => println!("Blinking Complete!"),
        Err(err) => println!("I have a blinking problem! {:?}", err),
    }
}
