// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;
use std::env;

struct Arguments {
    pin: u64,
    duration_ms: u32,
    period_ms: u32,
}

// Export a GPIO for use.  This will not fail if already exported
fn blink_my_led(led: u64, duration_ms: u32, period_ms: u32) -> sysfs_gpio::Result<()> {
    let my_led = Pin::new(led);
    my_led.with_exported(|| {
        try!(my_led.set_direction(Direction::Low));
        let iterations = duration_ms / period_ms / 2;
        for _ in 0..iterations {
            try!(my_led.set_value(0));
            sleep(Duration::from_millis(period_ms as u64));
            try!(my_led.set_value(1));
            sleep(Duration::from_millis(period_ms as u64));
        }
        try!(my_led.set_value(0));
        Ok(())
    })
}

fn print_usage() {
    println!("Usage: ./blinky <pin> <duration_ms> <period_ms>");
}

fn get_args() -> Option<Arguments> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 { return None; }
    let pin = match args[1].parse::<u64>() { Ok(pin) => pin, Err(_) => return None, };
    let duration_ms = match args[2].parse::<u32>() { Ok(ms) => ms, Err(_) => return None, };
    let period_ms = match args[3].parse::<u32>() { Ok(ms) => ms, Err(_) => return None, };
    Some(Arguments { pin: pin, duration_ms: duration_ms, period_ms: period_ms, })
}

fn main() {
    match get_args() {
        None => print_usage(),
        Some(args) => {
            match blink_my_led(args.pin, args.duration_ms, args.period_ms) {
                Ok(()) => println!("Success!"),
                Err(err) => println!("We have a blinking problem: {}", err),
            }
        },
    }
}
