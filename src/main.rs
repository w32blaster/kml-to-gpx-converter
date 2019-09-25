#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde_xml_rs::from_reader;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

mod coordinates;

const PATH_SOURCE_FILE: &str = "/Users/ilja.hamalainen/Downloads/DirectiosFromClarendonHotelToPrincessStreetCarPark.kml";

#[derive(Deserialize, Debug)]
struct KmlRoot {
    #[serde(rename="Document")]
    pub document: Document,
}

#[derive(Deserialize, Debug)]
struct Document {
    #[serde(rename="Placemark")]
    pub placemark: Vec<Placemark>,
    pub name: String,

}

#[derive(Deserialize, Debug)]
struct Placemark {
    pub name: String,
    #[serde(rename="styleUrl")]
    pub style_url: String,
    
    #[serde(rename="LineString", default)]
    pub line_strings: Vec<LineString>,

    #[serde(rename="Point", default)]
    pub points: Vec<Point>,
}

#[derive(Deserialize, Debug)]
struct LineString {
    pub tessellate: i8,
    pub coordinates: String,

    #[serde(default)]
    pub coordinates_parsed: Vec<coordinates::Coordinate>, // multiple coordinates
}

#[derive(Deserialize, Debug)]
struct Point {
    pub coordinates: String,

    #[serde(default)]
    pub coordinates_parsed: coordinates::Coordinate, // single tuple
}


fn main() {
    println!("Let's parse the source KML file");


    // 1. read the source file and parse it
    let res = read_source_file();
    let mut root: KmlRoot = match res {
        Ok(c) => from_reader(c.as_bytes()).unwrap(),
        Err(error) => {
            panic!("Problem opening the source file: {:?}", error)
        },
    };

    for placemark in &mut root.document.placemark {

        if placemark.points.len() > 0 {
            for mut point in &mut placemark.points {
                let mut coordinates: Vec<coordinates::Coordinate> = Vec::new();
                coordinates::parse_coordinates(String::from(&point.coordinates), &mut coordinates);
                point.coordinates_parsed = coordinates[0];
            }
        }
        
        if placemark.line_strings.len() > 0 {
            for mut lstrings in &mut placemark.line_strings {
               let mut coordinates: Vec<coordinates::Coordinate> = Vec::new();
               coordinates::parse_coordinates(String::from(&lstrings.coordinates), &mut coordinates);
               lstrings.coordinates_parsed = coordinates;
            }
        }
    }

    println!("{:?}", root);
}

// read the source file and returns its content as a string
fn read_source_file() -> std::io::Result<String> {
    let file = File::open(PATH_SOURCE_FILE).expect("Failed to open source file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

