// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::io;
use std::env;
use std::thread::sleep_ms;

fn poll(pin_num : u64) -> io::Result<()> {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.
    let input = Pin::new(pin_num);
    input.with_exported(|| {
        try!(input.set_direction(Direction::In));
        let mut prev_val : u8 = 255;
        loop {
            let val = try!(input.get_value());
            if val != prev_val {
                println!("Pin State: {}",
                         if val == 0 { "Low" } else { "High" });
                prev_val = val;
            }
            sleep_ms(10);
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./poll <pin>");
    } else {
        match args[1].parse::<u64>() {
            Ok(pin) => match poll(pin) {
                Ok(()) => println!("Polling Complete!"),
                Err(err) => println!("Error: {}", err),
            },
            Err(_) => println!("Usage: ./poll <pin>"),
        }
    }
}
