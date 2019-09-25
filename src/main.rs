
extern crate serde;
extern crate serde_xml_rs;
extern crate xml;

#[macro_use]
extern crate serde_derive;

mod commons;
mod source;
mod output;

fn main() {
    println!("Let's parse the source KML file");

    // 1. read the source file and parse it
    let root = source::parse_source();

    // 2. write data to the output GPX file 
    output::write_gpx_from(root);
}

