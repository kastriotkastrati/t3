use crate::tle;

const axis: u64 = 6_378_137;

trait CartesianToGeodetic {
    fn to_geodetic(cartesian_coordinates: &CartesianCoordinates) -> GeodeticCoordinates {
        return GeodeticCoordinates;
    }
}

pub struct BowringsMethod;
pub struct GeodeticCoordinates;
pub struct CartesianCoordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl CartesianToGeodetic for BowringsMethod {
    fn to_geodetic(cartesian_coordinates: &CartesianCoordinates) -> GeodeticCoordinates {
        return GeodeticCoordinates;
    }
}
impl CartesianCoordinates {
    pub fn new(from: [f64; 3]) -> Self {
        let x = from[0];
        let y = from[1];
        let z = from[2];
        return Self { x, y, z };
    }
}

pub fn determine_overhead(coordinates: GeodeticCoordinates) -> bool {
    return true;
}

pub fn calculate_elevation_angle() -> f64 {
    return 0.0;
}

pub fn to_coordinates(position: CartesianCoordinates) -> u8 {
    return 4;
}

pub fn main() {
    let data = tle::get_data::get_daily_predictions();
    let first = data.first().unwrap();
    let first_prediction = first.first().unwrap().prediction.position;
}
