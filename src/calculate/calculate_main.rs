// bowring's method:
// https://www.osti.gov/servlets/purl/231228
// https://gis.stackexchange.com/questions/265909/converting-from-ecef-to-geodetic-coordinates

// non-iterative appraoch + C code:
// Karl Osen. Accurate Conversion of Earth-Fixed Earth-Centered Coordinates to Geodetic Coordinates.
// [Research Report] Norwegian University of Science and Technology. 2017. ffhal-01704943v2f
// https://hal.science/hal-01704943v2/document

use crate::geolocation;
use crate::tle;
use crate::units;
use crate::units::Inner;

const AXIS: f64 = 6_378_137f64;
const FLATTENING: f64 = 0.00335281055;
const ALLOWED_ELEVATION_BOUND_UPPER: f64 = 110.0;
const ALLOWED_ELEVATION_BOUND_LOWER: f64 = 65.0;

trait CartesianToGeodetic {
    fn to_geodetic(cartesian_coordinates: &CartesianCoordinates) -> GeodeticCoordinates;
}

struct BowringsMethod;

#[derive(Debug)]
pub struct GeodeticCoordinates {
    pub longitude: units::AngleUnit,
    pub latitude: units::AngleUnit,
    pub altitude: f64,
}

#[derive(Debug)]
pub struct CartesianCoordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub unit: units::LengthUnit,
}

impl GeodeticCoordinates {
    fn new(longitude: f64, latitude: f64, altitude: f64) -> Self {
        return Self {
            altitude,
            latitude: units::AngleUnit::Radians(units::Radians(latitude)),
            longitude: units::AngleUnit::Radians(units::Radians(longitude)),
        };
    }

    fn radian_to_degrees(&self) -> Result<Self, &str> {
        let latitude = self.latitude.radian_to_degrees()?;
        let longitude = self.longitude.radian_to_degrees()?;

        return Ok(Self {
            altitude: self.altitude,
            latitude: units::AngleUnit::Degrees(latitude),
            longitude: units::AngleUnit::Degrees(longitude),
        });
    }
}

impl CartesianCoordinates {
    pub fn new(from: [f64; 3]) -> Self {
        let x = from[0];
        let y = from[1];
        let z = from[2];
        return Self {
            x,
            y,
            z,
            unit: units::LengthUnit::Km,
        };
    }

    fn from_km_to_m(&self) -> Result<Self, &str> {
        if let units::LengthUnit::M = self.unit {
            return Err("Data is in m format already");
        }

        let x = self.x * 1000f64;
        let y = self.y * 1000f64;
        let z = self.z * 1000f64;

        return Ok(Self {
            x,
            y,
            z,
            unit: units::LengthUnit::M,
        });
    }
}

impl CartesianToGeodetic for BowringsMethod {
    fn to_geodetic(cartesian_coordinates: &CartesianCoordinates) -> GeodeticCoordinates {
        let e2 = get_eccentricity_squared();
        let coordinate_x = cartesian_coordinates.x;
        let coordinate_y = cartesian_coordinates.y;
        let coordinate_z = cartesian_coordinates.z;
        let distance_from_z_axis = {
            let x = coordinate_x.powi(2);
            let y = coordinate_y.powi(2);
            let psquared = x + y;
            let p = psquared.sqrt();
            p
        };

        let longitude = coordinate_y.atan2(coordinate_x);
        let latitude = {
            let beta = (1.0 - FLATTENING) * (coordinate_z / distance_from_z_axis).atan();
            let numerator = coordinate_z + e2 * AXIS * beta.sin().powi(3);
            let denominator = distance_from_z_axis - e2 * AXIS * beta.cos().powi(3);
            let setup = numerator / denominator;
            let arctan = setup.atan();
            arctan
        };

        let altitude = {
            let n = AXIS / (1.0 - e2 * latitude.sin().powi(2)).sqrt();
            let h = distance_from_z_axis / latitude.cos() - n;
            h
        };

        let coordinates = GeodeticCoordinates::new(longitude, latitude, altitude);
        return coordinates;
    }
}

fn get_eccentricity_squared() -> f64 {
    let x = 2.0 * FLATTENING;
    let x2 = FLATTENING.powi(2);
    let y = x - x2;
    return y;
}

fn haversine_great_distance(
    user_latitude: &units::Radians,
    user_longitude: &units::Radians,
    satellite_latitude: &units::Radians,
    satellite_longitude: &units::Radians,
) -> f64 {
    let delta_latitude = satellite_latitude.inner() - user_latitude.inner();
    let delta_longitude = satellite_longitude.inner() - user_longitude.inner();
    let a = {
        let a1 = (delta_latitude / 2.0).sin().powi(2);
        let a2 = user_latitude.inner().cos();
        let a3 = satellite_latitude.inner().cos();
        let a4 = (delta_longitude / 2.0).sin().powi(2);
        let result = a1 + a2 * a3 * a4;
        result
    };

    let c = {
        let c1 = (1.0 - a).sqrt();
        let c2 = a.sqrt().atan2(c1);
        let result = 2.0 * c2;
        result
    };

    let d = AXIS * c;
    return d;
}

fn elevation_angle(
    haversine_great_distance: units::Radians,
    satellite_altitude: units::Radians,
) -> f64 {
    let angle = (haversine_great_distance.inner() / satellite_altitude.inner())
        .atan()
        .to_degrees();

    return 90.0 - angle;
}

pub fn calculate_overhead_satellites() -> Vec<(f64, String, f64, units::Degrees, units::Degrees)> {
    let data = tle::tle_main::get_daily_predictions();
    let user_geodetic_coordinates = geolocation::geolocation_main::get_user_geolocation();
    let user_latitude_radian = user_geodetic_coordinates
        .latitude
        .degrees_to_radian()
        .unwrap();

    let user_longitude_radian = user_geodetic_coordinates
        .longitude
        .degrees_to_radian()
        .unwrap();

    let closest_distances = data.into_iter().map(|prediction_at_minute| {
        let position = prediction_at_minute.prediction.position;
        let position = CartesianCoordinates::new(position);
        let position = position.from_km_to_m().unwrap();
        let geodetic_data = BowringsMethod::to_geodetic(&position);
        return (prediction_at_minute, geodetic_data);
    });

    let closest_distances = closest_distances.map(|(prediction, geodetic_data)| {
        let satellite_latitude_radians = geodetic_data.latitude.as_radians();
        let satellite_longitude_radians = geodetic_data.longitude.as_radians();
        let distance = haversine_great_distance(
            &user_latitude_radian,
            &user_longitude_radian,
            &satellite_latitude_radians,
            &satellite_longitude_radians,
        );
        return (prediction, geodetic_data, distance);
    });

    let closest_distances =
        closest_distances.filter_map(|(prediction, geodetic_data, distance)| {
            let elevation = elevation_angle(
                units::Radians(distance),
                units::Radians(geodetic_data.altitude),
            );

            if elevation > ALLOWED_ELEVATION_BOUND_UPPER {
                return None;
            };

            if elevation < ALLOWED_ELEVATION_BOUND_LOWER {
                return None;
            };

            let data = (
                distance,
                prediction.satellite_name.clone(),
                elevation,
                geodetic_data.latitude.as_degrees(),
                geodetic_data.longitude.as_degrees(),
            );
            return Some(data);
        });

    let closest_distances: Vec<(f64, String, f64, units::Degrees, units::Degrees)> =
        closest_distances.collect();

    return closest_distances;
}
