/*
    Field	Columns	Content	Example
1	01	Line number	1
2	03–07	Satellite catalog number	25544
3	08	Classification (U: unclassified, C: classified, S: secret) [11]	U
4	10–11	International Designator (last two digits of launch year)	98
5	12–14	International Designator (launch number of the year)	067
6	15–17	International Designator (piece of the launch)	A
7	19–20	Epoch year (last two digits of year)	08
8	21–32	Epoch (day of the year and fractional portion of the day)	264.51782528
9	34–43	First derivative of mean motion; the ballistic coefficient [12]	-.00002182
10	45–52	Second derivative of mean motion (decimal point assumed) [12]	00000-0
11	54–61	B*, the drag term, or radiation pressure coefficient (decimal point assumed) [12]	-11606-4
12	63–63	Ephemeris type (always zero; only used in undistributed TLE data) [13]	0
13	65–68	Element set number. Incremented when a new TLE is generated for this object.[12]	292
14	69	Checksum (modulo 10)	7
*/

pub struct TLEElements(sgp4::Elements);
impl TLEElements {
    pub fn new(data: sgp4::Elements) -> Self {
        return Self(data);
    }

    pub fn inner(&self) -> &sgp4::Elements {
        return &self.0;
    }
}
pub struct TLE(TLEElements);

impl Clone for TLEElements {
    fn clone(&self) -> Self {
        let data = &self.0;
        let name = data.object_name.clone();
        let international_designator_copied = data.international_designator.clone();
        let classification_copied = match data.classification {
            sgp4::Classification::Classified => sgp4::Classification::Classified,
            sgp4::Classification::Secret => sgp4::Classification::Secret,
            sgp4::Classification::Unclassified => sgp4::Classification::Unclassified,
        };
        let inner = sgp4::Elements {
            datetime: data.datetime,
            argument_of_perigee: data.argument_of_perigee,
            classification: classification_copied,
            drag_term: data.drag_term,
            eccentricity: data.eccentricity,
            object_name: name,
            element_set_number: data.element_set_number,
            ephemeris_type: data.ephemeris_type,
            inclination: data.inclination,
            international_designator: international_designator_copied,
            mean_anomaly: data.mean_anomaly,
            mean_motion: data.mean_motion,
            mean_motion_ddot: data.mean_motion_ddot,
            mean_motion_dot: data.mean_motion_dot,
            norad_id: data.norad_id,
            revolution_number: data.revolution_number,
            right_ascension: data.right_ascension,
        };
        return Self(inner);
    }
}

impl TLE {
    pub fn new(data: sgp4::Elements) -> Self {
        let tle_elements = TLEElements::new(data);
        let tle = TLE(tle_elements);
        return tle;
    }

    pub fn inner(&self) -> &TLEElements {
        return &self.0;
    }
}
