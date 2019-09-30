

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use crate::commons::UniversalPoint;

mod coordinates;

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

pub fn parse_source(file_path: String) -> (Vec<UniversalPoint>, Vec<UniversalPoint>) {

    // 1. Open the file and read it raw content, then parse it
    let res = read_source_file(file_path);
    let root: KmlRoot = match res {
        Ok(c) => serde_xml_rs::from_reader(c.as_bytes()).unwrap(),
        Err(error) => {
            panic!("Problem opening the source file: {:?}", error)
        },
    };

    // 2. Build the vector of universal points
    let mut uni_points_track: Vec<UniversalPoint> = Vec::new();
    let mut uni_points_pois: Vec<UniversalPoint> = Vec::new();
    for placemark in &root.document.placemark {

        // collect al the POIs
        if placemark.points.len() > 0 {
            for point in &placemark.points {
                let coordinates = coordinates::parse_coordinates(point.coordinates.clone());
                for c in coordinates {
                    uni_points_pois.push(UniversalPoint{
                        longitude: c.longitude, 
                        latitude: c.latitude, 
                        altitude: c.altitude,
                        description: "".to_string(),
                        name: (*placemark).name.to_string(),
                    });
                }
            }    
        }
        
        // Collect all the points for a track
        if placemark.line_strings.len() > 0 {
            for lstrings in &placemark.line_strings {
               let coordinates = coordinates::parse_coordinates(lstrings.coordinates.clone());
                for c in coordinates {
                    uni_points_track.push(UniversalPoint{
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

    return (uni_points_track, uni_points_pois);
}

// read the source file and returns its content as a string
fn read_source_file(file_path: String) -> std::io::Result<String> {
    let file = File::open(file_path).expect("Failed to open source file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}