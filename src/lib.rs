//! A Rust library to find the nearest AWS region to a given location.
//!
//! Especially useful when you run code at the edge and want fast access to
//! regional AWS services, e.g. Cloudflare Workers accessing DynamoDB global
//! tables.

#![deny(clippy::all, clippy::nursery)]
#![deny(nonstandard_style, rust_2018_idioms)]
#![deny(missing_docs, missing_debug_implementations)]

use std::fmt;

use geoutils::Location;
use ordered_float::OrderedFloat;

/// An AWS region.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AwsRegion {
    /// Africa (Cape Town)
    AfSouth1,

    /// Asia Pacific (Hong Kong)
    ApEast1,

    /// Asia Pacific (Tokyo)
    ApNortheast1,

    /// Asia Pacific (Seoul)
    ApNortheast2,

    /// Asia Pacific (Osaka)
    ApNortheast3,

    /// Asia Pacific (Mumbai)
    ApSouth1,

    /// Asia Pacific (Singapore)
    ApSoutheast1,

    /// Asia Pacific (Sydney)
    ApSoutheast2,

    /// Asia Pacific (Jakarta)
    ApSoutheast3,

    /// Canada (Central)
    CaCentral1,

    /// AWS China (Beijing)
    CnNorth1,

    /// AWS China (Ningxia)
    CnNorthwest1,

    /// Europe (Frankfurt)
    EuCentral1,

    /// Europe (Stockholm)
    EuNorth1,

    /// Europe (Milan)
    EuSouth1,

    /// Europe (Ireland)
    EuWest1,

    /// Europe (London)
    EuWest2,

    /// Europe (Paris)
    EuWest3,

    /// Middle East (Bahrain)
    MeSouth1,

    /// South America (São Paulo)
    SaEast1,

    /// US East (N. Virginia)
    UsEast1,

    /// US East (Ohio)
    UsEast2,

    /// US West (N. California)
    UsWest1,

    /// US West (Oregon)
    UsWest2,

    /// AWS GovCloud (US-East)
    UsGovEast1,

    /// AWS GovCloud (US-West)
    UsGovWest1,
}

impl AwsRegion {
    /// Returns an iterator over all regions.
    pub fn iter() -> impl Iterator<Item = Self> {
        const REGIONS: [AwsRegion; 26] = [
            AwsRegion::AfSouth1,
            AwsRegion::ApEast1,
            AwsRegion::ApNortheast1,
            AwsRegion::ApNortheast2,
            AwsRegion::ApNortheast3,
            AwsRegion::ApSouth1,
            AwsRegion::ApSoutheast1,
            AwsRegion::ApSoutheast2,
            AwsRegion::ApSoutheast3,
            AwsRegion::CaCentral1,
            AwsRegion::CnNorth1,
            AwsRegion::CnNorthwest1,
            AwsRegion::EuCentral1,
            AwsRegion::EuNorth1,
            AwsRegion::EuSouth1,
            AwsRegion::EuWest1,
            AwsRegion::EuWest2,
            AwsRegion::EuWest3,
            AwsRegion::MeSouth1,
            AwsRegion::SaEast1,
            AwsRegion::UsEast1,
            AwsRegion::UsEast2,
            AwsRegion::UsWest1,
            AwsRegion::UsWest2,
            AwsRegion::UsGovEast1,
            AwsRegion::UsGovWest1,
        ];
        REGIONS.iter().copied()
    }

    /// Returns the name of the region.
    pub const fn name(&self) -> &'static str {
        match *self {
            AwsRegion::AfSouth1 => "af-south-1",
            AwsRegion::ApEast1 => "ap-east-1",
            AwsRegion::ApNortheast1 => "ap-northeast-1",
            AwsRegion::ApNortheast2 => "ap-northeast-2",
            AwsRegion::ApNortheast3 => "ap-northeast-3",
            AwsRegion::ApSouth1 => "ap-south-1",
            AwsRegion::ApSoutheast1 => "ap-southeast-1",
            AwsRegion::ApSoutheast2 => "ap-southeast-2",
            AwsRegion::ApSoutheast3 => "ap-southeast-3",
            AwsRegion::CaCentral1 => "ca-central-1",
            AwsRegion::CnNorth1 => "cn-north-1",
            AwsRegion::CnNorthwest1 => "cn-northwest-1",
            AwsRegion::EuCentral1 => "eu-central-1",
            AwsRegion::EuNorth1 => "eu-north-1",
            AwsRegion::EuSouth1 => "eu-south-1",
            AwsRegion::EuWest1 => "eu-west-1",
            AwsRegion::EuWest2 => "eu-west-2",
            AwsRegion::EuWest3 => "eu-west-3",
            AwsRegion::MeSouth1 => "me-south-1",
            AwsRegion::SaEast1 => "sa-east-1",
            AwsRegion::UsEast1 => "us-east-1",
            AwsRegion::UsEast2 => "us-east-2",
            AwsRegion::UsWest1 => "us-west-1",
            AwsRegion::UsWest2 => "us-west-2",
            AwsRegion::UsGovEast1 => "us-gov-east-1",
            AwsRegion::UsGovWest1 => "us-gov-west-1",
        }
    }

    /// Returns the location of the region.
    // Coordinates taken from https://gist.github.com/tobilg/ba6a5e1635478d13efdea5c1cd8227de
    pub fn location(&self) -> Location {
        match *self {
            AwsRegion::AfSouth1 => Location::new(-33.9648017883, 18.6016998291), // Cape Town, South Africa
            AwsRegion::ApEast1 => Location::new(22.308901, 113.915001),          // Hong Kong, China
            AwsRegion::ApNortheast1 => Location::new(35.764702, 140.386002),     // Tokyo, Japan
            AwsRegion::ApNortheast2 => Location::new(37.46910095214844, 126.45099639892578), // Seoul, South Korea
            AwsRegion::ApNortheast3 => Location::new(34.42729949951172, 135.24400329589844), // Osaka, Japan
            AwsRegion::ApSouth1 => Location::new(19.0886993408, 72.8678970337),  // Mumbai, India
            AwsRegion::ApSoutheast1 => Location::new(1.35019, 103.994003),       // Singapore
            AwsRegion::ApSoutheast2 => Location::new(-33.94609832763672, 151.177001953125), // Syndney, Australia
            AwsRegion::ApSoutheast3 => Location::new(-6.125556, 106.655833),     // Jakarta, Indonesia
            AwsRegion::CaCentral1 => Location::new(45.470556, -73.740833),       // Montreal, Canada
            AwsRegion::CnNorth1 => Location::new(40.080101013183594, 116.58499908447266), // Beijing, China
            AwsRegion::CnNorthwest1 => Location::new(38.321667, 106.3925),       // Yinchuan, China
            AwsRegion::EuCentral1 => Location::new(50.033333, 8.570556),         // Frankfurt, Germany
            AwsRegion::EuNorth1 => Location::new(59.651901245117, 17.918600082397), // Stockholm, Sweden
            AwsRegion::EuSouth1 => Location::new(45.6306, 8.72811),              // Milan, Italy
            AwsRegion::EuWest1 => Location::new(53.421299, -6.27007),            // Dublin, Ireland
            AwsRegion::EuWest2 => Location::new(51.4775, -0.461389),             // London, United Kingdom
            AwsRegion::EuWest3 => Location::new(49.012798, 2.55),                // Paris, France
            AwsRegion::MeSouth1 => Location::new(26.27079963684082, 50.63359832763672), // Manama, Bahrain
            AwsRegion::SaEast1 => Location::new(-23.435556, -46.473056),         // São Paulo, Brazil
            AwsRegion::UsEast1 => Location::new(38.9445, -77.4558029),           // Ashburn, Virginia, USA
            AwsRegion::UsEast2 => Location::new(39.958993960575775, -83.00219086148725), // Columbus, Ohio, USA
            AwsRegion::UsWest1 => Location::new(37.61899948120117, -122.375),    // San Francisco, California, USA
            AwsRegion::UsWest2 => Location::new(45.540394, -122.949825),         // Hillsboro, Oregon, USA
            AwsRegion::UsGovEast1 => Location::new(38.9445, -77.4558029),        // Ashburn, Virginia, USA
            AwsRegion::UsGovWest1 => Location::new(37.61899948120117, -122.375), // San Francisco, California, USA
        }
    }

    /// Returns the distance in meters between the region and the given location.
    pub fn distance_to(&self, to: &Location) -> f64 {
        self.location().haversine_distance_to(to).meters()
    }
}

impl fmt::Display for AwsRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Finds the nearest AWS region to the given location.
pub fn find_region<T: Into<f64>>(latitude: T, longitude: T) -> AwsRegion {
    let location = Location::new(latitude.into(), longitude.into());

    AwsRegion::iter()
        .min_by_key(|region| OrderedFloat(region.distance_to(&location)))
        .unwrap()
}

/// Finds the nearest AWS region from a list of regions.
/// Panics if regions is empty.
pub fn find_region_from_list<T: Into<f64>>(latitude: T, longitude: T, regions: &[AwsRegion]) -> AwsRegion {
    let location = Location::new(latitude.into(), longitude.into());

    *regions
        .iter()
        .min_by_key(|region| OrderedFloat(region.distance_to(&location)))
        .expect("regions must not be empty")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_region_name() {
        assert_eq!(AwsRegion::EuCentral1.name(), "eu-central-1");
        assert_eq!(AwsRegion::CnNorthwest1.name(), "cn-northwest-1");
    }

    #[test]
    fn test_region_to_string() {
        assert_eq!(AwsRegion::EuCentral1.to_string(), "eu-central-1");
        assert_eq!(AwsRegion::CnNorthwest1.to_string(), "cn-northwest-1");
    }

    #[test]
    fn test_region_location() {
        let location = AwsRegion::AfSouth1.location();
        assert_eq!(location, Location::new(-33.9648017883, 18.6016998291));
        assert_eq!(location.latitude(), -33.9648017883);
        assert_eq!(location.longitude(), 18.6016998291);
    }

    #[test]
    fn test_region_distance_to() {
        let region = AwsRegion::EuWest1;
        assert_eq!(region.distance_to(&region.location()), 0.0);
        assert_eq!(region.distance_to(&AwsRegion::EuWest2.location()), 448_890.249);
        assert_eq!(region.distance_to(&AwsRegion::EuWest3.location()), 784_967.795);
    }

    #[test]
    fn test_region_iter() {
        assert_eq!(AwsRegion::iter().next().unwrap().name(), "af-south-1");
        assert_eq!(AwsRegion::iter().last().unwrap().name(), "us-gov-west-1");
        assert_eq!(AwsRegion::iter().count(), 26);
    }

    #[test]
    fn test_find_region() {
        struct Test {
            city: &'static str,
            latitude: f64,
            longitude: f64,
            region: AwsRegion,
        }

        let tests = vec![
            Test {
                city: "Hamburg",
                latitude: 53.5511,
                longitude: 9.9937,
                region: AwsRegion::EuCentral1,
            },
            Test {
                city: "Manchester",
                latitude: 53.4808,
                longitude: -2.2426,
                region: AwsRegion::EuWest2,
            },
            Test {
                city: "Las Vegas",
                latitude: 36.1699,
                longitude: -115.1398,
                region: AwsRegion::UsWest1,
            },
            Test {
                city: "Boston",
                latitude: 42.3601,
                longitude: -71.0589,
                region: AwsRegion::CaCentral1,
            },
            Test {
                city: "Kyoto",
                latitude: 35.0116,
                longitude: 135.7681,
                region: AwsRegion::ApNortheast3,
            },
            Test {
                city: "Cairo",
                latitude: 30.0444,
                longitude: 31.2357,
                region: AwsRegion::MeSouth1,
            },
        ];

        for t in tests.iter() {
            let region = find_region(t.latitude, t.longitude);
            assert_eq!(region, t.region, "{}", t.city);
        }
    }

    #[test]
    fn test_find_region_from_list() {
        struct Test {
            city: &'static str,
            latitude: f64,
            longitude: f64,
            list: Vec<AwsRegion>,
            region: AwsRegion,
        }

        let tests = vec![
            Test {
                city: "Hamburg",
                latitude: 53.5511,
                longitude: 9.9937,
                list: vec![AwsRegion::EuWest1, AwsRegion::EuCentral1, AwsRegion::EuSouth1],
                region: AwsRegion::EuCentral1,
            },
            Test {
                city: "Manchester",
                latitude: 53.4808,
                longitude: -2.2426,
                list: vec![AwsRegion::UsEast1, AwsRegion::UsWest1],
                region: AwsRegion::UsEast1,
            },
            Test {
                city: "Las Vegas",
                latitude: 36.1699,
                longitude: -115.1398,
                list: vec![AwsRegion::UsGovEast1, AwsRegion::UsGovWest1],
                region: AwsRegion::UsGovWest1,
            },
            Test {
                city: "Boston",
                latitude: 42.3601,
                longitude: -71.0589,
                list: vec![AwsRegion::CaCentral1, AwsRegion::CaCentral1],
                region: AwsRegion::CaCentral1,
            },
            Test {
                city: "Kyoto",
                latitude: 35.0116,
                longitude: 135.7681,
                list: vec![AwsRegion::ApEast1, AwsRegion::ApNortheast3],
                region: AwsRegion::ApNortheast3,
            },
            Test {
                city: "Cairo",
                latitude: 30.0444,
                longitude: 31.2357,
                list: vec![AwsRegion::EuWest2, AwsRegion::EuWest3],
                region: AwsRegion::EuWest3,
            },
        ];

        for t in tests.iter() {
            let region = find_region_from_list(t.latitude, t.longitude, &t.list);
            assert_eq!(region, t.region, "{}", t.city);
        }
    }

    #[test]
    #[should_panic(expected = "regions must not be empty")]
    fn test_find_region_from_empty_list() {
        find_region_from_list(0, 0, &[]);
    }
}
