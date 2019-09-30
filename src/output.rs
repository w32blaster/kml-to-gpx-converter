

use crate::commons::UniversalPoint;

const XML_PREFIX: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<gpx xmlns=\"http://www.topografix.com/GPX/1/1\" xmlns:gpsies=\"https://www.gpsies.com/GPX/1/0\" creator=\"AllTrails https://www.alltrails.com\" 
  version=\"1.1\" xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" 
  xsi:schemaLocation=\"http://www.topografix.com/GPX/1/1 http://www.topografix.com/GPX/1/1/gpx.xsd https://www.gpsies.com/GPX/1/0 https://www.gpsies.com/gpsies.xsd\">
  <metadata>
    <name>Test route</name>
    <time>2019-09-20T11:29:31Z</time>
  </metadata>
  ";

const XML_TRK_START: &str = "        <trk>
    <name>test route</name>
    <trkseg>";
const XML_END: &str = "</gpx>";

pub fn write_gpx_from(data: Vec<UniversalPoint>, pois: Vec<UniversalPoint>) {

    let mut s = String::new();
    s.push_str(XML_PREFIX);

    // tracks
    s.push_str(XML_TRK_START);
    for d in &data {
        let top:String = format!("      <trkpt lat=\"{}\" lon=\"{}\">\n", d.latitude, d.longitude);
        s.push_str(&top[..]);
        s.push_str("        <ele>0</ele>\n        <time>2010-01-01T09:17:14Z</time>\n");
        if d.name != "" {
            s.push_str("        <name>");
            s.push_str(&d.name[..]);
            s.push_str("</name>\n");
        }
        if d.description != "" {
            s.push_str("        <desc>");
            s.push_str(&d.description[..]);
            s.push_str("</desc>\n");
        }
        s.push_str("      </trkpt>\n");
    };
    s.push_str("      </trkseg>\n</trk>\n");
    
    // POIs
    for d in &pois {
        let top:String = format!("      <wpt lat=\"{}\" lon=\"{}\">\n", d.latitude, d.longitude);
        s.push_str(&top[..]);
        s.push_str("        <ele>0</ele>\n");
        if d.name != "" {
            s.push_str("        <name>");
            s.push_str(&d.name[..]);
            s.push_str("</name>\n");
        }
        if d.description != "" {
            s.push_str("        <desc>");
            s.push_str(&d.description[..]);
            s.push_str("</desc>\n");
        }
        s.push_str("      </wpt>\n");
    };


    s.push_str(XML_END);

    println!("{}", s);
}