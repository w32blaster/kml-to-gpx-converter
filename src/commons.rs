// Universal point is a point that can be understod by both systems
#[derive(Deserialize, Debug)]
pub struct UniversalPoint {
   pub longitude: f32, // between  −180 and 180
   pub latitude: f32, // between -90 and 90
   pub altitude: i32, // in meters above sea level, optional
   pub name: String,
   pub description: String,
}
