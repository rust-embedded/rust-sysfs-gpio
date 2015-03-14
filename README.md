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
extern crate gpio;

use sysfs_gpio::core as gpio;
use std::sys::unix::timer::Timer;

// export a GPIO for use.  This will not fail
// if already exported
fn blink_my_led(u64 duration_ms, u64 period_ms) -> GPIOResult {
    let my_led = try!(gpio::export(21));
    try!(my_led.set_direction(gpio::Direction::Output));
    let timer = try!(Timer::new());
    let iterations = duration_ms / period_ms / 2;
    for _ in 0.. {
        try!(my_led.set_value(0));
        timer.sleep(200); // ms
        try!(my_led.set_value(1));
        timer.sleep(200); // ms
    }

    // NOTE: we do not unexport here.  Handling the
    // error case is tricky and means we cannot
    // use try!.  This is where the higher-level
    // API can work out nicely.
    return Ok(());
}

match blink_my_led(5000, 200) {
    Ok(()) => println!("Blinking Complete!"),
    Err(err) => println!("I have a blinking problem! {:?}");
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

Copyright (c) 2015, Paul Osborne.

This library is released as Open Source under the terms of the MIT
License.  See the LICENSE file for additional details.
