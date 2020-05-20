extern crate coremidi;

use coremidi::{
    Client,
    PacketList,
    Properties,
};

fn main() {
    let client = Client::new("Example Client").unwrap();

    let callback = |packet_list: &PacketList| {
        println!("{}", packet_list);
    };

    // Creates a virtual destination, then gets its properties
    let destination = client.virtual_destination("Example Destination", callback).unwrap();

    println!("Created Virtual Destination...");

    // How to get a property
    let name = destination.get_property_string(Properties::name()).unwrap();
    println!("With Name: {}", name);

    // How to set a property
    destination.set_property_boolean(Properties::private(), true).unwrap();
}
