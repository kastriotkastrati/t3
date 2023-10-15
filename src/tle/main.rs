use crate::tle::structure;
use sgp4;
use ureq;

const TLE_URL_HOST: &'static str = "celestrak.org";
const TLE_URL_ROUTE: &'static str = "/NORAD/elements";
const TLE_URL_PARAMS: &'static str = "/gp.php?GROUP=stations";

#[derive(Debug)]
pub struct TleError;

fn get_url() -> String {
    return format!(
        "https://{}{}{}",
        TLE_URL_HOST, TLE_URL_ROUTE, TLE_URL_PARAMS
    );
}

fn fetch_tle_data(url: &str) -> Result<String, TleError> {
    let data_from_tle_source = ureq::get(&url)
        .call()
        .map_err(|_| TleError)?
        .into_string()
        .map_err(|_| TleError)?;

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

pub fn main() -> Vec<structure::TLE> {
    let url = get_url();
    let tle_source_data = fetch_tle_data(&url).unwrap();
    let tle_source_data_parsed = parse_tle_data(&tle_source_data);
    return tle_source_data_parsed;
}
