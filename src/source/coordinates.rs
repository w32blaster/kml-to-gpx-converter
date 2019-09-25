
#[derive(Deserialize, Debug, Default, Copy, Clone)]
pub struct Coordinate {
   pub longitude: f32, // between  −180 and 180
   pub latitude: f32, // between -90 and 90
   pub altitude: i32, // in meters above sea level, optional
}

// we expect here something like "-2.9293918,54.5709437,0"
// Please refer to unit tests below
pub fn parse_coordinates(raw_list: String) -> Vec<Coordinate> {
    
    let mut coordinates: Vec<Coordinate> = Vec::new();
    
    if raw_list.len() == 0 {
        return coordinates;
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
        coordinates.push(Coordinate{longitude: lon, latitude: lat, altitude: alt});
    }

    return coordinates;
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


#[test]
fn test_few_coordinates() {

    // Given
    let xml = "
          -2.80211,54.63673,0
          -2.80207,54.63673,0
          -2.80172,54.63678,0
          -2.80137,54.63679,0
          -2.80044,54.63674,0
          -2.79985,54.63674,0
          -2.79923,54.63682,0
          -2.79858,54.63697,0
          -2.79746,54.63731,0
          -2.79613,54.63776,0
    ".to_string();

    // When
    let coordinates = parse_coordinates(xml);

    // Then
    assert_eq!(coordinates.len(), 10);
    
    // and
    assert_eq!(coordinates[0].longitude, -2.80211);
    assert_eq!(coordinates[0].latitude, 54.63673);
    assert_eq!(coordinates[0].altitude, 0);

    assert_eq!(coordinates[1].longitude, -2.80207);
    assert_eq!(coordinates[1].latitude, 54.63673);
    assert_eq!(coordinates[1].altitude, 0);

    assert_eq!(coordinates[2].longitude, -2.80172);
    assert_eq!(coordinates[2].latitude, 54.63678);
    assert_eq!(coordinates[2].altitude, 0);

    assert_eq!(coordinates[3].longitude, -2.80137);
    assert_eq!(coordinates[3].latitude, 54.63679);
    assert_eq!(coordinates[3].altitude, 0);

    assert_eq!(coordinates[4].longitude, -2.80044);
    assert_eq!(coordinates[4].latitude, 54.63674);
    assert_eq!(coordinates[4].altitude, 0);

    assert_eq!(coordinates[5].longitude, -2.79985);
    assert_eq!(coordinates[5].latitude, 54.63674);
    assert_eq!(coordinates[5].altitude, 0);

    assert_eq!(coordinates[6].longitude, -2.79923);
    assert_eq!(coordinates[6].latitude, 54.63682);
    assert_eq!(coordinates[6].altitude, 0);

    assert_eq!(coordinates[7].longitude, -2.79858);
    assert_eq!(coordinates[7].latitude, 54.63697);
    assert_eq!(coordinates[7].altitude, 0);

    assert_eq!(coordinates[8].longitude, -2.79746);
    assert_eq!(coordinates[8].latitude, 54.63731);
    assert_eq!(coordinates[8].altitude, 0);

    assert_eq!(coordinates[9].longitude, -2.79613);
    assert_eq!(coordinates[9].latitude, 54.63776);
    assert_eq!(coordinates[9].altitude, 0);
}
