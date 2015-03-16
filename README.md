rust-sysfs-gpio
===============

rust-sysfs-gpio is a rust library/crate providing access to the Linux
sysfs GPIO interface (https://www.kernel.org/doc/Documentation).  It
seeks to provide an API that is safe, convenient, and efficient.

Many devices such as the Raspberry Pi or Beaglebone Black provide
userspace access to a number of GPIO peripherals.  The standard kernel
API for providing access to these GPIOs is via sysfs.

Example/API
-----------

The follow example shows the low-level API.  This API maps directly to
the functionality provided by the sysfs GPIO interface.

```rust
#![feature(io)]
#![feature(old_io)]
#![feature(std_misc)]

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

```

Features
--------

The following features are planned for the library:

- [ ] Support for exporting a GPIO
- [ ] Support for unexporting a GPIO
- [ ] Support for setting the direction of a GPIO (in/out)
- [ ] Support for reading the value of a GPIO input
- [ ] Support for writing the value of a GPIO output
- [ ] Support for configuring whether a pin is active low/high
- [ ] Support for configuring interrupts on GPIO
- [ ] Support for polling on GPIO with configured interrupt

License
-------

Copyright (c) 2015, Paul Osborne <ospbau@gmail.com>

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/license/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option.  This file may not be copied, modified, or distributed
except according to those terms.
