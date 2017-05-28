#[cfg(feature = "tokio")]
extern crate futures;
#[cfg(feature = "tokio")]
extern crate sysfs_gpio;
#[cfg(feature = "tokio")]
extern crate tokio_core;

#[cfg(feature = "tokio")]
use futures::{Future, Stream};
#[cfg(feature = "tokio")]
use sysfs_gpio::{Direction, Edge, Pin};
#[cfg(feature = "tokio")]
use std::env;
#[cfg(feature = "tokio")]
use tokio_core::reactor::Core;

#[cfg(feature = "tokio")]
fn stream(pin_nums: Vec<u64>) -> sysfs_gpio::Result<()> {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.
    let pins: Vec<_> = pin_nums.iter().map(|&p| (p, Pin::new(p))).collect();
    let mut l = try!(Core::new());
    let handle = l.handle();
    for &(i, ref pin) in pins.iter() {
        try!(pin.export());
        try!(pin.set_direction(Direction::In));
        try!(pin.set_edge(Edge::BothEdges));
        handle.spawn(try!(pin.get_value_stream(&handle))
                         .for_each(move |val| {
                                       println!("Pin {} changed value to {}", i, val);
                                       Ok(())
                                   })
                         .map_err(|_| ()));
    }
    // Wait forever for events
    loop {
        l.turn(None)
    }
}

#[cfg(feature = "tokio")]
fn main() {
    let pins: Vec<u64> = env::args()
        .skip(1)
        .map(|a| a.parse().expect("Pins must be specified as integers"))
        .collect();
    if pins.is_empty() {
        println!("Usage: ./tokio <pin> [pin ...]");
    } else {
        stream(pins).unwrap();
    }
}

#[cfg(not(feature = "tokio"))]
fn main() {
    println!("This example requires the `tokio` feature to be enabled.");
}
