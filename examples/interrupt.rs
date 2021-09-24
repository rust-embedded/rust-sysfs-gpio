// Copyright 2015, Paul Osborne <osbpau@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/license/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option.  This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::io::prelude::*;
use std::io::stdout;
use sysfs_gpio::{Direction, Edge, Pin};

fn interrupt(pin: u64) -> sysfs_gpio::Result<()> {
    let input = Pin::new(pin);
    input.with_exported(|| {
        input.set_direction(Direction::In)?;
        input.set_edge(Edge::BothEdges)?;
        let mut poller = input.get_poller()?;
        loop {
            match poller.poll(1000)? {
                Some(value) => println!("{}", value),
                None => {
                    let mut stdout = stdout();
                    stdout.write_all(b".")?;
                    stdout.flush()?;
                }
            }
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./interrupt <pin>");
    } else {
        match args[1].parse::<u64>() {
            Ok(pin) => match interrupt(pin) {
                Ok(()) => println!("Interrupting Complete!"),
                Err(err) => println!("Error: {}", err),
            },
            Err(_) => println!("Usage: ./interrupt <pin>"),
        }
    }
}
