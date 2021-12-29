#![deny(clippy::all, clippy::nursery)]
#![deny(nonstandard_style, rust_2018_idioms)]

use geoutils::Location;
use ordered_float::OrderedFloat;
use strum::{EnumProperty, IntoEnumIterator};

mod region;
use region::AwsRegion;

pub fn find_region_nearby<T: Into<f64>>(latitude: T, longitude: T) -> AwsRegion {
    let src = Location::new(latitude.into(), longitude.into());

    AwsRegion::iter()
        .min_by_key(|region| {
            let dst = Location::new(
                // FIXME
                region.get_str("latitude").unwrap().parse::<f64>().unwrap(),
                region.get_str("longitude").unwrap().parse::<f64>().unwrap(),
            );
            OrderedFloat(src.haversine_distance_to(&dst).meters())
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    struct Test {
        city: &'static str,
        latitude: f64,
        longitude: f64,
        region: &'static str,
    }

    fn tests() -> Vec<Test> {
        vec![
            Test {
                city: "Hamburg",
                latitude: 53.5511,
                longitude: 9.9937,
                region: "eu-central-1",
            },
            Test {
                city: "Manchester",
                latitude: 53.4808,
                longitude: -2.2426,
                region: "eu-west-2",
            },
            Test {
                city: "Las Vegas",
                latitude: 36.1699,
                longitude: -115.1398,
                region: "us-west-1",
            },
            Test {
                city: "Boston",
                latitude: 42.3601,
                longitude: -71.0589,
                region: "ca-central-1",
            },
            Test {
                city: "Kyoto",
                latitude: 35.0116,
                longitude: 135.7681,
                region: "ap-northeast-3",
            },
            Test {
                city: "Cairo",
                latitude: 30.0444,
                longitude: 31.2357,
                region: "me-south-1",
            },
        ]
    }

    #[test]
    fn test_find_region_nearby() {
        for t in tests().iter() {
            let region = find_region_nearby(t.latitude, t.longitude);
            assert_eq!(region.to_string(), t.region, "{}", t.city);
        }
    }
}
