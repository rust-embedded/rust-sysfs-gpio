extern crate sysfs_gpio;
use sysfs_gpio::{Direction, Edge, Pin};

extern crate mio;
use mio::{Events, Poll, PollOpt, Ready, Token};

const GPIO_EDGE: Token = Token(0);

fn get_evented_pin(pin: u16) -> Result<sysfs_gpio::AsyncPinPoller, sysfs_gpio::Error> {
    let pin = Pin::new(pin.into());
    pin.set_direction(Direction::In)?;
    pin.set_edge(Edge::BothEdges)?;
    pin.get_async_poller()
}

fn main() {
    // Construct a new `Poll` handle as well as the `Events` we'll store into
    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(8);

    let evented_pin = get_evented_pin(49).unwrap();

    // Register the Pin's Evented interface with `Poll`
    poll.register(
        &evented_pin,
        GPIO_EDGE,
        Ready::readable() | Ready::writable(),
        PollOpt::edge(),
    ).unwrap();

    // Linux gives you an event by default so disarm the first one
    poll.poll(&mut events, None).unwrap();

    loop {
        // wait for events, which all should be pin toggles
        poll.poll(&mut events, None).unwrap();

        for event in &events {
            match event.token() {
                GPIO_EDGE => println!("Edge Detected"),
                _ => println!("Unhandled event!"),
            }
        }
    }
}
