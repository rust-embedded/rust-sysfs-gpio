rust-sysfs-gpio
===============

[![Build Status](https://travis-ci.org/posborne/rust-sysfs-gpio.svg?branch=master)](https://travis-ci.org/posborne/rust-sysfs-gpio)
[![Version](https://img.shields.io/crates/v/sysfs-gpio.svg)](https://crates.io/crates/sysfs-gpio)
[![License](https://img.shields.io/crates/l/sysfs-gpio.svg)](https://github.com/posborne/rust-sysfs-gpio/blob/master/README.md#license)

- [API Documentation](http://posborne.github.io/rust-sysfs-gpio/)

rust-sysfs-gpio is a rust library/crate providing access to the Linux
sysfs GPIO interface (https://www.kernel.org/doc/Documentation).  It
seeks to provide an API that is safe, convenient, and efficient.

Many devices such as the Raspberry Pi or Beaglebone Black provide
userspace access to a number of GPIO peripherals.  The standard kernel
API for providing access to these GPIOs is via sysfs.

Install/Use
-----------

To use `sysfs_gpio`, first add this to your `Cargo.toml`:

```toml
[dependencies]
sysfs_gpio = "*"
```

Then, add this to your crate root:

```rust
extern crate sysfs_gpio;
```

Example/API
-----------

Blinking an LED:

```rust
extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep_ms;

fn main() {
    let my_led = Pin::new(127); // number depends on chip, etc.
    my_led.with_exported(|| {
        loop {
            my_led.set_value(0).unwrap();
            sleep_ms(200);
            my_led.set_value(1).unwrap();
            sleep_ms(200);
        }
    }).unwrap();
}
```

More Examples:

- [Blink an LED](examples/blinky.rs)
- [Poll a GPIO Input](examples/poll.rs)
- [Receive interrupt on GPIO Change](examples/interrupt.rs)

Features
--------

The following features are planned for the library:

- [x] Support for exporting a GPIO
- [x] Support for unexporting a GPIO
- [x] Support for setting the direction of a GPIO (in/out)
- [x] Support for reading the value of a GPIO input
- [x] Support for writing the value of a GPIO output
- [ ] Support for configuring whether a pin is active low/high
- [x] Support for configuring interrupts on GPIO
- [x] Support for polling on GPIO with configured interrupt

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

Running the Example
-------------------

Cross-compiling can be done by specifying an appropriate target.  You
can then move that to your device by whatever means and run it.

```
$ cargo build --target=arm-unknown-linux-gnueabihf --example blinky
  ...
$ scp target/arm-unknown-linux-gnueabihf/debug/examples/blinky ...
```

License
-------

```
Copyright (c) 2015, Paul Osborne <ospbau@gmail.com>

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/license/LICENSE-2.0> or the MIT license
<LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
option.  This file may not be copied, modified, or distributed
except according to those terms.
```
