// Copyright (c) 2020.  The sysfs-gpio Authors.

use futures::future::join_all;
use futures::StreamExt;
use std::env;
use sysfs_gpio::{Direction, Edge, Pin};

async fn monitor_pin(pin: Pin) -> Result<(), sysfs_gpio::Error> {
    pin.export()?;
    pin.set_direction(Direction::In)?;
    pin.set_edge(Edge::BothEdges)?;
    let mut gpio_events = pin.get_value_stream()?;
    while let Some(evt) = gpio_events.next().await {
        let val = evt.unwrap();
        println!("Pin {} changed value to {}", pin.get_pin_num(), val);
    }
    Ok(())
}

async fn stream(pin_nums: Vec<u64>) {
    // NOTE: this currently runs forever and as such if
    // the app is stopped (Ctrl-C), no cleanup will happen
    // and the GPIO will be left exported.  Not much
    // can be done about this as Rust signal handling isn't
    // really present at the moment.  Revisit later.
    join_all(pin_nums.into_iter().map(|p| {
        let pin = Pin::new(p);
        tokio::task::spawn(monitor_pin(pin))
    }))
    .await;
}

#[tokio::main]
async fn main() {
    let pins: Vec<u64> = env::args()
        .skip(1)
        .map(|a| a.parse().expect("Pins must be specified as integers"))
        .collect();
    if pins.is_empty() {
        println!("Usage: ./tokio <pin> [pin ...]");
    } else {
        stream(pins).await;
    }
}
