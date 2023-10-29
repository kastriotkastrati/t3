use crate::tle::structure;
use ureq;

const TLE_URL_HOST: &str = "celestrak.org";
const TLE_URL_ROUTE: &str = "/NORAD/elements";
const TLE_URL_PARAMS: &str = "/gp.php?GROUP=stations";

pub struct PredictionAtMinute {
    pub satellite_name: String,
    pub minute: f64,
    pub prediction: sgp4::Prediction,
}

impl PredictionAtMinute {
    pub fn new(satellite_name: &str, minute: f64, prediction: sgp4::Prediction) -> Self {
        let satellite_name = satellite_name.to_owned();
        return Self {
            satellite_name,
            minute,
            prediction,
        };
    }
}

fn get_url() -> String {
    return format!(
        "https://{}{}{}",
        TLE_URL_HOST, TLE_URL_ROUTE, TLE_URL_PARAMS
    );
}

fn fetch_tle_data(url: &str) -> Result<String, structure::TleError> {
    let data_from_tle_source = ureq::get(url)
        .call()
        .map_err(|_| structure::TleError)?
        .into_string()
        .map_err(|_| structure::TleError)?;

    return Ok(data_from_tle_source);
}

fn parse_tle_data(raw_data: &str) -> Vec<structure::TLE> {
    let idk = sgp4::parse_3les(raw_data).unwrap();
    let tles: Vec<structure::TLE> = idk
        .into_iter()
        .map(|x| {
            let tle = structure::TLE::new(x);
            return tle;
        })
        .collect();

    return tles;
}

fn get_prediction(data: &structure::TLE) -> Vec<PredictionAtMinute> {
    let elements = data.inner().inner();
    let name = elements.object_name.as_ref().unwrap();
    let constants = sgp4::Constants::from_elements(elements).unwrap();
    let minutes_in_a_day = 24 * 60;
    let minutes = 0..minutes_in_a_day;
    let predictions_by_minute: Vec<PredictionAtMinute> = minutes
        .map(|minute| {
            let minute = minute as f64;
            let prediction = constants.propagate(minute).unwrap();
            let prediction_at_minute = PredictionAtMinute::new(name, minute, prediction);
            return prediction_at_minute;
        })
        .collect();

    return predictions_by_minute;
}

pub fn get_daily_predictions() -> Vec<PredictionAtMinute> {
    let url = get_url();
    let tle_source_data = fetch_tle_data(&url).unwrap();
    let tle_source_data_parsed = parse_tle_data(&tle_source_data);
    let full_day_predictions: Vec<PredictionAtMinute> = tle_source_data_parsed
        .iter()
        .flat_map(get_prediction)
        .collect();

    return full_day_predictions;
}
