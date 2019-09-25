
struct Coordinate {
   pub longitude: f32, // between  −180 and 180
   pub latitude: f32, // between -90 and 90
   pub altitude: i32, // in meters above sea level, optional
}

fn parse_coordinates(raw_list: String) -> Vec<Coordinate> {
    let mut v: Vec<Coordinate> = Vec::new();

    if raw_list.len() == 0 {
        return v;
    }

    for row in raw_list.split("\n") {
        let row = row.trim();
        if row.len() == 0 {
            continue
        }

        let part:Vec<&str> = row.split(",").collect();
        let lon:f32 = String::from(part[0]).parse::<f32>().unwrap();
        let lat:f32 = String::from(part[1]).parse::<f32>().unwrap();
        let alt:i32 = String::from(part[2]).parse::<i32>().unwrap();
        v.push(Coordinate{longitude: lon, latitude: lat, altitude: alt});
    }

    return v;
}

#[test]
fn test_empty_list() {

    // Given:
    let xml = "".to_string();

    // When:
    let coordinates = parse_coordinates(xml);

    // Then:
    assert_eq!(coordinates.len(), 0);
}

#[test]
fn test_one_coordinate() {

    // Given
    let xml = "
    -2.9293918,54.5709437,0
    ".to_string();

    // When
    let coordinates = parse_coordinates(xml);

    // Then
    assert_eq!(coordinates.len(), 1);
    
    // and
    assert_eq!(coordinates[0].longitude, -2.9293918);
    assert_eq!(coordinates[0].latitude, 54.5709437);
    assert_eq!(coordinates[0].altitude, 0);
}

