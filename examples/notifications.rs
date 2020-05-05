extern crate coremidi;
extern crate crossterm;

use crossterm::event::{read, Event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    println!("Logging Client Notifications - Press any key to exit.");

    let _client = coremidi::Client::new_with_notifications("example-client", print_notification).unwrap();

    loop_until_keys_pressed();
}

fn print_notification(notification: &coremidi::Notification) {
    println!("Received Notification: {:?} \r", notification);
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