#[cfg(feature = "async-tokio")]
extern crate futures;
#[cfg(feature = "async-tokio")]
extern crate sysfs_gpio;
#[cfg(feature = "async-tokio")]
extern crate tokio;

#[cfg(feature = "async-tokio")]
use std::env;

#[cfg(feature = "async-tokio")]
use futures::{lazy, Future, Stream};

#[cfg(feature = "async-tokio")]
use sysfs_gpio::{Direction, Edge, Pin};

#[cfg(feature = "async-tokio")]
fn stream(pin_nums: Vec<u64>) -> sysfs_gpio::Result<()> {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.
    let pins: Vec<_> = pin_nums.iter().map(|&p| (p, Pin::new(p))).collect();
    let task = lazy(move || {
        for &(i, ref pin) in pins.iter() {
            pin.export().unwrap();
            pin.set_direction(Direction::In).unwrap();
            pin.set_edge(Edge::BothEdges).unwrap();
            tokio::spawn(
                pin.get_value_stream()
                    .unwrap()
                    .for_each(move |val| {
                        println!("Pin {} changed value to {}", i, val);
                        Ok(())
                    })
                    .map_err(|_| ()),
            );
        }
        Ok(())
    });
    tokio::run(task);

    Ok(())
}

#[cfg(feature = "async-tokio")]
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

#[cfg(not(feature = "async-tokio"))]
fn main() {
    println!("This example requires the `tokio` feature to be enabled.");
}
