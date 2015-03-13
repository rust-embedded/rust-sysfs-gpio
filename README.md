rust-sysfs-gpio
===============

rust-sysfs-gpio is a rust library/crate providing access to the Linux
sysfs GPIO interface (https://www.kernel.org/doc/Documentation).  It
seeks to provide an API that is safe, convenient, and efficient.

Many devices such as the Raspberry Pi or Beaglebone Black provide
userspace access to a number of GPIO peripherals.  The standard kernel
API for providing access to these GPIOs is via sysfs.

Example
-------

```rust
// TO BE CONTINUED...
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
