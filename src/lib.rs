use geoutils::Location;
use ordered_float::OrderedFloat;

mod regions;
use regions::{AwsRegion, AWS_REGIONS};

pub fn find_region_nearby(latitude: f32, longitude: f32) -> AwsRegion {
    let src = Location::new(latitude, longitude);

    *AWS_REGIONS
        .iter()
        .min_by_key(|region| {
            let dst = Location::new(region.latitude, region.longitude);
            OrderedFloat(src.haversine_distance_to(&dst).meters())
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let region_near_hamburg = find_region_nearby(53.5511, 9.9937);
        assert_eq!(region_near_hamburg.name, "eu-central-1");

        let region_near_sf = find_region_nearby(37.7749, -122.4194);
        assert_eq!(region_near_sf.name, "us-west-1");
    }
}
