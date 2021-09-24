sysfs_gpio
==========

[![Build Status](https://github.com/rust-embedded/rust-sysfs-gpio/workflows/CI/badge.svg)](https://github.com/rust-embedded/rust-sysfs-gpio/actions)
[![Version](https://img.shields.io/crates/v/sysfs-gpio.svg)](https://crates.io/crates/sysfs-gpio)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.46+-blue.svg)
[![License](https://img.shields.io/crates/l/sysfs-gpio.svg)](https://github.com/rust-embedded/rust-sysfs-gpio/blob/master/README.md#license)

- [API Documentation](https://docs.rs/sysfs_gpio)

The `sysfs_gpio` crate provides access to the Linux sysfs GPIO interface
(https://www.kernel.org/doc/Documentation/gpio/sysfs.txt). It seeks to provide
an API that is safe, convenient, and efficient and supports exporting,
unexporting, reading, writing and waiting for interrupts on pins.

Many devices such as the Raspberry Pi or Beaglebone Black provide
userspace access to a number of GPIO peripherals.  The standard kernel
API for providing access to these GPIOs is via sysfs.

You might want to also check out the
[gpio-utils Project](https://github.com/rust-embedded/gpio-utils) for a
convenient way to associate names with pins and export them as part of system
boot.  That project uses this library.

## Install/Use

To use `sysfs_gpio`, first add this to your `Cargo.toml`:

```toml
[dependencies]
sysfs_gpio = "0.6"
```

Then, add this to your crate root:

```rust
use sysfs_gpio;
```

## Example/API

Blinking an LED:

```rust
use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let my_led = Pin::new(127); // number depends on chip, etc.
    my_led.with_exported(|| {
        my_led.set_direction(Direction::Out).unwrap();
        loop {
            my_led.set_value(0).unwrap();
            sleep(Duration::from_millis(200));
            my_led.set_value(1).unwrap();
            sleep(Duration::from_millis(200));
        }
    }).unwrap();
}
```

More Examples:

- [Blink an LED](examples/blinky.rs)
- [Poll a GPIO Input](examples/poll.rs)
- [Receive interrupt on GPIO Change](examples/interrupt.rs)
- [Poll several pins asynchronously with Tokio](examples/tokio.rs)
- [gpio-utils Project (uses most features)](https://github.com/rust-embedded/gpio-utils)

## Features

The following features are planned for the library:

- [x] Support for exporting a GPIO
- [x] Support for unexporting a GPIO
- [x] Support for setting the direction of a GPIO (in/out)
- [x] Support for reading the value of a GPIO input
- [x] Support for writing the value of a GPIO output
- [ ] Support for configuring whether a pin is active low/high
- [x] Support for configuring interrupts on GPIO
- [x] Support for polling on GPIO with configured interrupt
- [x] Support for asynchronous polling using `mio` or `tokio` (requires
      enabling the `mio-evented` or `async-tokio` crate features, respectively)

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.46.0 and up.  It *might*
compile with older versions but that may change in any new patch release.

## Cross Compiling

Most likely, the machine you are running on is not your development
machine (although it could be).  In those cases, you will need to
cross-compile.  The [rust-cross guide](https://github.com/japaric/rust-cross)
provides excellent, detailed instructions for cross-compiling.

## Running the Example

Cross-compiling can be done by specifying an appropriate target.  You
can then move that to your device by whatever means and run it.

```
$ cargo build --target=arm-unknown-linux-gnueabihf --example blinky
$ scp target/arm-unknown-linux-gnueabihf/debug/examples/blinky ...
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, the [Embedded Linux Team][team], promises
to intervene to uphold that code of conduct.

[CoC]: CODE_OF_CONDUCT.md
[team]: https://github.com/rust-embedded/wg#the-embedded-linux-team
