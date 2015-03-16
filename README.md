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

Cross Compiling
---------------

Most likely, the machine you are running on is not your development
machine (although it could be).  In those cases, you will need to
cross-compile.  The following basic instructions should work for the
raspberry pi or beaglebone black:

1. Install rust and cargo
2. Install an appropriate cross compiler.  On an Ubuntu system, this
   can be done by doing `sudo apt-get install g++-arm-linux-gnueabihf`.
3. Build or install rust for your target.  This is necessary in order
   to have libstd available for your target.  For arm-linux-gnueabihf,
   you can find binaries at https://github.com/japaric/ruststrap.
   With this approach or building it yourself, you will need to copy
   the ${rust}/lib/rustlib/arm-unknown-linux-gnueabihf to your system
   rust library folder (it is namespaced by triple, so it shouldn't
   break anything).
4. Tell cargo how to link by adding the lines below to your
   ~/.cargo/config file.
5. Run your build `cargo build --target=arm-unknown-linux-gnueabi`.

The following snippet added to my ~/.cargo/config worked for me:

```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```

License
-------

Copyright (c) 2015, Paul Osborne <ospbau@gmail.com>

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/license/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option.  This file may not be copied, modified, or distributed
except according to those terms.
