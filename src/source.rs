

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::commons::UniversalPoint;

mod coordinates;

const PATH_SOURCE_FILE: &str = "/Users/ilja.hamalainen/Downloads/DirectiosFromClarendonHotelToPrincessStreetCarPark.kml";

#[derive(Deserialize, Debug)]
struct KmlRoot {
    #[serde(rename="Document")]
    document: Document,
}

#[derive(Deserialize, Debug)]
struct Document {
    #[serde(rename="Placemark")]
    placemark: Vec<Placemark>,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Placemark {
    name: String,
    #[serde(rename="styleUrl")]
    style_url: String,
    
    #[serde(rename="LineString", default)]
    line_strings: Vec<LineString>,

    #[serde(rename="Point", default)]
    points: Vec<Point>,
}

#[derive(Deserialize, Debug)]
struct LineString {
    tessellate: i8,
    coordinates: String,
}

#[derive(Deserialize, Debug)]
struct Point {
    coordinates: String,
}

pub fn parse_source() -> Vec<UniversalPoint> {

    // 1. Open the file and read it raw content, then parse it
    let res = read_source_file();
    let root: KmlRoot = match res {
        Ok(c) => serde_xml_rs::from_reader(c.as_bytes()).unwrap(),
        Err(error) => {
            panic!("Problem opening the source file: {:?}", error)
        },
    };

   // 2. Build the vector of universal points
    let mut uni_points: Vec<UniversalPoint> = Vec::new();

    for placemark in &root.document.placemark {
        if placemark.points.len() > 0 {

            for point in &placemark.points {
                let coordinates = coordinates::parse_coordinates(String::from(&point.coordinates));
                for c in coordinates {
                    uni_points.push(UniversalPoint{
                        longitude: c.longitude, 
                        latitude: c.latitude, 
                        altitude: c.altitude,
                        description: "".to_string(),
                        name: (*placemark).name.to_string(),
                    });
                }
            }
        }
        
        if placemark.line_strings.len() > 0 {
            for lstrings in &placemark.line_strings {
               let coordinates = coordinates::parse_coordinates(String::from(&lstrings.coordinates));
                for c in coordinates {
                    uni_points.push(UniversalPoint{
                        longitude: c.longitude, 
                        latitude: c.latitude, 
                        altitude: c.altitude,
                        description: "".to_string(),
                        name: "".to_string(),
                    });
                }
            }
        }
    }

    return uni_points;
}

// read the source file and returns its content as a string
fn read_source_file() -> std::io::Result<String> {
    let file = File::open(PATH_SOURCE_FILE).expect("Failed to open source file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}