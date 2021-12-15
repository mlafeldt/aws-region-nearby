use geoutils::Location;
use ordered_float::OrderedFloat;

#[derive(Debug, Clone, Copy)]
pub struct AwsRegion {
    name: &'static str,
    iata_code: &'static str,
    latitude: f64,
    longitude: f64,
}

// Values from https://github.com/tobilg/aws-edge-locations#json-lookup

const US_EAST_1: AwsRegion = AwsRegion {
    name: "us-east-1",
    iata_code: "IAD",
    latitude: 38.9445,
    longitude: -77.4558029,
};
const EU_CENTRAL_1: AwsRegion = AwsRegion {
    name: "eu-central-1",
    iata_code: "FRA",
    latitude: 50.033333,
    longitude: 8.570556,
};

const AWS_REGIONS: [AwsRegion; 2] = [US_EAST_1, EU_CENTRAL_1];

pub fn find_region_nearby(latitude: f64, longitude: f64) -> String {
    let src = Location::new(latitude, longitude);

    let region = AWS_REGIONS
        .iter()
        .min_by_key(|region| {
            let dst = Location::new(region.latitude, region.longitude);
            let distance = src.haversine_distance_to(&dst).meters();
            dbg!(region.name, distance / 1000.0);
            OrderedFloat(distance)
        })
        .unwrap();

    region.name.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let region_near_hamburg = find_region_nearby(53.5511, 9.9937);
        assert_eq!(region_near_hamburg, "eu-central-1");

        let region_near_sf = find_region_nearby(37.7749, -122.4194);
        assert_eq!(region_near_sf, "us-east-1");
    }
}
