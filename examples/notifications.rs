extern crate coremidi;
extern crate crossterm;

use crossterm::event::{read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    println!("Logging Client Notifications");
    println!("");
    println!("Press any key to exit.");
    println!("");

    loop_until_keys_pressed();
}

fn loop_until_keys_pressed() {
    enable_raw_mode().expect("Couldn't enable terminal raw mode");
    loop {
        let event = read().expect("Couldn't read next terminal event");

        match event {
            Event::Key(_) => break,
            _ => { }
        }
    }
    disable_raw_mode().expect("Couldn't disable terminal raw mode");
}