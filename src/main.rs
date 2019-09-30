
extern crate serde;
extern crate serde_xml_rs;

#[macro_use]
extern crate serde_derive;

use std::env;

mod commons;
mod source;
mod output;

fn main() {

    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(x) => {

            // 1. read the source file and parse it
            let (track_points, pois) = source::parse_source(x.clone());

            // 2. write data to the output GPX file 
            output::write_gpx_from(track_points, pois);
        },
        None => println!("The path for the source file is not specified. Please, call the application with an following argument, such as $ {} myfile.klm", args[0]),
    }

}

