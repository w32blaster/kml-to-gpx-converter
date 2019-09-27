
extern crate serde;
extern crate serde_xml_rs;

#[macro_use]
extern crate serde_derive;

mod commons;
mod source;
mod output;

fn main() {

    // 1. read the source file and parse it
    let (track_points, pois) = source::parse_source();

    // 2. write data to the output GPX file 
    output::write_gpx_from(track_points, pois);
}

