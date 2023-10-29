use crate::units;
use ureq;

const ACCEPT_HEADER_KEY: &str = "Accept";
const ACCEPT_HEADER_VALUE: &str = "application/json";
const IP_URI: &str = "https://ifconfig.me";
const LOCATION_URI: &str = "http://ip-api.com/json";

pub struct Ip(String);

#[derive(Debug)]
pub struct Location {
    pub latitude: units::AngleUnit,
    pub longitude: units::AngleUnit,
    pub city: String,
    pub country: String,
}

impl Ip {
    pub fn new(ip: &str) -> Self {
        return Self(ip.to_owned());
    }

    pub fn inner(&self) -> &str {
        return &self.0;
    }
}

pub fn get_ip() -> Result<Ip, ureq::Error> {
    let ip = ureq::get(IP_URI)
        .set(ACCEPT_HEADER_KEY, ACCEPT_HEADER_VALUE)
        .call()?
        .into_string()?;

    let ip = Ip::new(&ip);
    return Ok(ip);
}

pub fn get_geolocation(ip: &Ip) -> Result<Location, ureq::Error> {
    let ip = ip.inner();
    let get_location_url_full = format!("{}/{}", LOCATION_URI, ip);
    let location_data_call = ureq::get(&get_location_url_full)
        .set(ACCEPT_HEADER_KEY, ACCEPT_HEADER_VALUE)
        .call()?
        .into_string()?;

    let location_data_parsed: Result<serde_json::Value, serde_json::Error> =
        serde_json::from_str(&location_data_call);

    let location_data_parsed = location_data_parsed.unwrap();
    let latitude_string = location_data_parsed["lat"].to_string();
    let longitude_string = location_data_parsed["lon"].to_string();
    let latitude = latitude_string.parse::<f64>().unwrap();
    let longitude = longitude_string.parse::<f64>().unwrap();
    let latitude = units::Degrees(latitude);
    let longitude = units::Degrees(longitude);

    let location_data = Location {
        latitude: units::AngleUnit::Degrees(latitude),
        longitude: units::AngleUnit::Degrees(longitude),
        city: location_data_parsed["city"].to_string(),
        country: location_data_parsed["country"].to_string(),
    };

    return Ok(location_data);
}

pub fn get_user_geolocation() -> Location {
    let ip = get_ip().expect("No IP could be found");
    let location = get_geolocation(&ip).expect("Could not find geolocation");
    return location;
}
