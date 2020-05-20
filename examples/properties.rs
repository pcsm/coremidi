extern crate coremidi;

use coremidi::{
    Client,
    PacketList,
    properties,
};

fn main() {
    let client = Client::new("Example Client").unwrap();

    let callback = |packet_list: &PacketList| {
        println!("{}", packet_list);
    };

    // Creates a virtual destination, then gets its properties
    let destination = client.virtual_destination("Example Destination", callback).unwrap();

    println!("Created Virtual Destination...");

    // Getting a property with a convenience accessor
    let name = destination.name().unwrap();
    println!(" - Name: {}", name);

    // Setting and getting a property that doesn't have a convenience accessor
    destination.set_property_boolean(properties::PRIVATE, true).unwrap();
    let private = destination.get_property_boolean(properties::PRIVATE).unwrap();
    println!(" - Private: {}", private);

    // Setting and getting a private property
    // See https://developer.apple.com/documentation/coremidi/midiobjectref for details on private properties
    destination.set_property_string("com_coremidi_APrivateProperty", "have a great day!").unwrap();
    let custom_val = destination.get_property_string("com_coremidi_APrivateProperty").unwrap();
    println!(" - Custom Property: {}", custom_val);
}
