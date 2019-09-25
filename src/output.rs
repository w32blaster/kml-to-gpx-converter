

use crate::commons::UniversalPoint;

#[derive(Serialize, Debug)]
#[serde(rename="gpx")]
struct Gpx {

    #[serde(rename="metadata")]
    metadata: Metadata,
    trk: Trk,
}

#[derive(Serialize, Debug)]
struct Metadata {
    name: String,
    time: String, // format: 2019-09-20T11:29:31Z
}

#[derive(Serialize, Debug)]
struct Trk {
    name: String,
    
    // #[serde(rename="trkpt", default)]
    // trkseg: Vec<Trkpt>,
}

#[derive(Serialize, Debug)]
struct Trkpt {
    lat: f32,
    lon: f32,
    elem: f32,
    time: String, // 2010-01-01T00:00:20Z
    name: String,
    desc: String,
}

pub fn write_gpx_from(data: Vec<UniversalPoint>) {

    let xml_prefix = r#"<?xml version="1.0" encoding="UTF-8" ?>"#;

    let mut vector_trkpt: Vec<Trkpt> = Vec::new();

    for d in &data {
        vector_trkpt.push(Trkpt{
            lat: d.latitude,
            lon: d.longitude,
            time: "2010-01-01T00:00:20Z".to_string(),
            name: d.name.to_string(),
            desc: d.description.to_string(),
            elem: 0.0,
        })
    };

    let gpx = Gpx{
        metadata: Metadata{
            name: "Test GPX route".to_string(),
            time: "2019-09-20T11:29:31Z".to_string(),
        },
        trk: Trk{
            name: "My Route".to_string(),
            //trkseg: Vec::new(),
        },
    };

    let reserialized_item = serde_xml_rs::to_string(&gpx).unwrap();
    let oput = format!("{}\0{}", xml_prefix, reserialized_item);
    println!("{}", oput);
}