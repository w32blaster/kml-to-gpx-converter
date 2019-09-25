
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;


mod source;

fn main() {
    println!("Let's parse the source KML file");


    // 1. read the source file and parse it
    let root = source::parse_source();
    println!("{:?}", root);
}

