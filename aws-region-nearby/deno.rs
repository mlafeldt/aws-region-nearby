use std::fmt;
use std::str::FromStr;

use geoutils::Location;

use crate::aws::AwsRegion;
use crate::Error;

/// A Deno Deploy region. Based on <https://deno.com/deploy/docs/regions>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DenoRegion {
    /// Taiwan
    AsiaEast1,

    /// Hong Kong
    AsiaEast2,

    /// Tokyo
    AsiaNortheast1,

    /// Osaka
    AsiaNortheast2,

    /// Seoul
    AsiaNortheast3,

    /// Mumbai
    AsiaSouth1,

    /// Delhi
    AsiaSouth2,

    /// Singapore
    AsiaSoutheast1,

    /// Jakarta
    AsiaSoutheast2,

    /// Sydney
    AustraliaSoutheast1,

    /// Melbourne
    AustraliaSoutheast2,

    /// Warsaw
    EuropeCentral2,

    /// Finland
    EuropeNorth1,

    /// Belgium
    EuropeWest1,

    /// London
    EuropeWest2,

    /// Frankfurt
    EuropeWest3,

    /// Netherlands
    EuropeWest4,

    /// Zurich
    EuropeWest6,

    /// Milan
    EuropeWest8,

    /// Paris
    EuropeWest9,

    /// Tel Aviv
    MeWest1,

    /// Madrid
    EuropeSouthwest1,

    /// Montréal
    NorthamericaNortheast1,

    /// Toronto
    NorthamericaNortheast2,

    /// São Paulo
    SouthamericaEast1,

    /// Chile
    SouthamericaWest1,

    /// Iowa
    UsCentral1,

    /// South Carolina
    UsEast1,

    /// North Virginia
    UsEast4,

    /// Ohio
    UsEast5,

    /// Texas
    UsSouth1,

    /// Oregon
    UsWest1,

    /// California
    UsWest2,

    /// Utah
    UsWest3,

    /// Nevada
    UsWest4,
}

impl DenoRegion {
    /// Returns an iterator over all regions.
    pub fn iter() -> impl Iterator<Item = Self> {
        const REGIONS: [DenoRegion; 35] = [
            DenoRegion::AsiaEast1,
            DenoRegion::AsiaEast2,
            DenoRegion::AsiaNortheast1,
            DenoRegion::AsiaNortheast2,
            DenoRegion::AsiaNortheast3,
            DenoRegion::AsiaSouth1,
            DenoRegion::AsiaSouth2,
            DenoRegion::AsiaSoutheast1,
            DenoRegion::AsiaSoutheast2,
            DenoRegion::AustraliaSoutheast1,
            DenoRegion::AustraliaSoutheast2,
            DenoRegion::EuropeCentral2,
            DenoRegion::EuropeNorth1,
            DenoRegion::EuropeWest1,
            DenoRegion::EuropeWest2,
            DenoRegion::EuropeWest3,
            DenoRegion::EuropeWest4,
            DenoRegion::EuropeWest6,
            DenoRegion::EuropeWest8,
            DenoRegion::EuropeWest9,
            DenoRegion::MeWest1,
            DenoRegion::EuropeSouthwest1,
            DenoRegion::NorthamericaNortheast1,
            DenoRegion::NorthamericaNortheast2,
            DenoRegion::SouthamericaEast1,
            DenoRegion::SouthamericaWest1,
            DenoRegion::UsCentral1,
            DenoRegion::UsEast1,
            DenoRegion::UsEast4,
            DenoRegion::UsEast5,
            DenoRegion::UsSouth1,
            DenoRegion::UsWest1,
            DenoRegion::UsWest2,
            DenoRegion::UsWest3,
            DenoRegion::UsWest4,
        ];
        REGIONS.iter().copied()
    }

    /// Returns the name of the region.
    pub const fn name(&self) -> &'static str {
        match *self {
            Self::AsiaEast1 => "asia-east1",
            Self::AsiaEast2 => "asia-east2",
            Self::AsiaNortheast1 => "asia-northeast1",
            Self::AsiaNortheast2 => "asia-northeast2",
            Self::AsiaNortheast3 => "asia-northeast3",
            Self::AsiaSouth1 => "asia-south1",
            Self::AsiaSouth2 => "asia-south2",
            Self::AsiaSoutheast1 => "asia-southeast1",
            Self::AsiaSoutheast2 => "asia-southeast2",
            Self::AustraliaSoutheast1 => "australia-southeast1",
            Self::AustraliaSoutheast2 => "australia-southeast2",
            Self::EuropeCentral2 => "europe-central2",
            Self::EuropeNorth1 => "europe-north1",
            Self::EuropeWest1 => "europe-west1",
            Self::EuropeWest2 => "europe-west2",
            Self::EuropeWest3 => "europe-west3",
            Self::EuropeWest4 => "europe-west4",
            Self::EuropeWest6 => "europe-west6",
            Self::EuropeWest8 => "europe-west8",
            Self::EuropeWest9 => "europe-west9",
            Self::MeWest1 => "me-west1",
            Self::EuropeSouthwest1 => "europe-southwest1",
            Self::NorthamericaNortheast1 => "northamerica-northeast1",
            Self::NorthamericaNortheast2 => "northamerica-northeast2",
            Self::SouthamericaEast1 => "southamerica-east1",
            Self::SouthamericaWest1 => "southamerica-west1",
            Self::UsCentral1 => "us-central1",
            Self::UsEast1 => "us-east1",
            Self::UsEast4 => "us-east4",
            Self::UsEast5 => "us-east5",
            Self::UsSouth1 => "us-south1",
            Self::UsWest1 => "us-west1",
            Self::UsWest2 => "us-west2",
            Self::UsWest3 => "us-west3",
            Self::UsWest4 => "us-west4",
        }
    }

    /// Returns the location of the region.
    // Coordinates taken from https://simplemaps.com/data/world-cities
    // TODO: share city coordinates with AwsRegion
    pub const fn location(&self) -> Location {
        match *self {
            Self::AsiaEast1 => Location::new_const(25.0478, 121.5319), // assuming Taipei, Taiwan
            Self::AsiaEast2 => Location::new_const(22.3069, 114.1831), // Hong Kong, China
            Self::AsiaNortheast1 => Location::new_const(35.6839, 139.7744), // Tokyo, Japan
            Self::AsiaNortheast2 => Location::new_const(34.7520, 135.4582), // Osaka, Japan
            Self::AsiaNortheast3 => Location::new_const(37.5600, 126.9900), // Seoul, South Korea
            Self::AsiaSouth1 => Location::new_const(19.0758, 72.8775), // Mumbai, India
            Self::AsiaSouth2 => Location::new_const(28.6667, 77.2167), // Delhi, India
            Self::AsiaSoutheast1 => Location::new_const(1.3000, 103.8000), // Singapore
            Self::AsiaSoutheast2 => Location::new_const(-6.2146, 106.8451), // Jakarta, Indonesia
            Self::AustraliaSoutheast1 => Location::new_const(-33.8650, 151.2094), // Sydney, Australia
            Self::AustraliaSoutheast2 => Location::new_const(-37.8136, 144.963), // Melbourne, Australia
            Self::EuropeCentral2 => Location::new_const(52.2300, 21.0111), // Warsaw, Poland
            Self::EuropeNorth1 => Location::new_const(60.1756, 24.9342), // assuming Helsinki, Finland
            Self::EuropeWest1 => Location::new_const(50.8353, 4.3314), // assuming Brussels, Belgium
            Self::EuropeWest2 => Location::new_const(51.5072, -0.1275), // London, United Kingdom
            Self::EuropeWest3 => Location::new_const(50.1136, 8.6797), // Frankfurt, Germany
            Self::EuropeWest4 => Location::new_const(52.3667, 4.8833), // assuming Amsterdam, Netherlands
            Self::EuropeWest6 => Location::new_const(47.3744, 8.5411), // Zurich, Switzerland
            Self::EuropeWest8 => Location::new_const(45.4669, 9.1900), // Milan, Italy
            Self::EuropeWest9 => Location::new_const(48.8566, 2.3522), // Paris, France
            Self::MeWest1 => Location::new_const(32.0800, 34.7800),    // Tel Aviv, Israel
            Self::EuropeSouthwest1 => Location::new_const(40.4167, -3.7167), // Madrid, Spain
            Self::NorthamericaNortheast1 => Location::new_const(45.5089, -73.5617), // Montréal, Canada
            Self::NorthamericaNortheast2 => Location::new_const(43.7417, -79.3733), // Toronto, Canada
            Self::SouthamericaEast1 => Location::new_const(-23.5504, -46.6339), // São Paulo, Brazil
            Self::SouthamericaWest1 => Location::new_const(-33.4500, -70.6667), // assuming Santiago, Chile
            Self::UsCentral1 => Location::new_const(41.5725, -93.6105), // assuming Des Moines, IA
            Self::UsEast1 => Location::new_const(34.0376, -80.9037),   // assuming Columbia, SC
            Self::UsEast4 => Location::new_const(39.0300, -77.4711),   // assuming Ashburn, VA
            Self::UsEast5 => Location::new_const(39.9862, -82.9850),   // assuming Columbus, OH
            Self::UsSouth1 => Location::new_const(30.3004, -97.7522),  // assuming Austin, TX
            Self::UsWest1 => Location::new_const(45.5272, -122.9361),  // assuming Hillsboro, OR
            Self::UsWest2 => Location::new_const(37.7562, -122.4430),  // assuming San Francisco, CA
            Self::UsWest3 => Location::new_const(40.7777, -111.9306),  // assuming Salt Lake City, UT
            Self::UsWest4 => Location::new_const(39.1512, -119.7474),  // assuming Carson City, NV
        }
    }

    /// Returns the distance in meters between the region and the given location.
    pub fn distance_to(&self, to: &Location) -> f64 {
        self.location().haversine_distance_to(to).meters()
    }

    /// Finds the nearest AWS region from a list of regions.
    ///
    /// # Panics
    ///
    /// Panics if regions is empty.
    pub fn find_region_from_list(&self, regions: &[AwsRegion]) -> AwsRegion {
        crate::find_region_from_list(self.location().latitude(), self.location().longitude(), regions)
    }
}

impl From<DenoRegion> for AwsRegion {
    fn from(region: DenoRegion) -> Self {
        crate::find_region(region.location().latitude(), region.location().longitude())
    }
}

impl fmt::Display for DenoRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

impl FromStr for DenoRegion {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_ref() {
            "asia-east1" => Ok(Self::AsiaEast1),
            "asia-east2" => Ok(Self::AsiaEast2),
            "asia-northeast1" => Ok(Self::AsiaNortheast1),
            "asia-northeast2" => Ok(Self::AsiaNortheast2),
            "asia-northeast3" => Ok(Self::AsiaNortheast3),
            "asia-south1" => Ok(Self::AsiaSouth1),
            "asia-south2" => Ok(Self::AsiaSouth2),
            "asia-southeast1" => Ok(Self::AsiaSoutheast1),
            "asia-southeast2" => Ok(Self::AsiaSoutheast2),
            "australia-southeast1" => Ok(Self::AustraliaSoutheast1),
            "australia-southeast2" => Ok(Self::AustraliaSoutheast2),
            "europe-central2" => Ok(Self::EuropeCentral2),
            "europe-north1" => Ok(Self::EuropeNorth1),
            "europe-west1" => Ok(Self::EuropeWest1),
            "europe-west2" => Ok(Self::EuropeWest2),
            "europe-west3" => Ok(Self::EuropeWest3),
            "europe-west4" => Ok(Self::EuropeWest4),
            "europe-west6" => Ok(Self::EuropeWest6),
            "europe-west8" => Ok(Self::EuropeWest8),
            "europe-west9" => Ok(Self::EuropeWest9),
            "me-west1" => Ok(Self::MeWest1),
            "europe-southwest1" => Ok(Self::EuropeSouthwest1),
            "northamerica-northeast1" => Ok(Self::NorthamericaNortheast1),
            "northamerica-northeast2" => Ok(Self::NorthamericaNortheast2),
            "southamerica-east1" => Ok(Self::SouthamericaEast1),
            "southamerica-west1" => Ok(Self::SouthamericaWest1),
            "us-central1" => Ok(Self::UsCentral1),
            "us-east1" => Ok(Self::UsEast1),
            "us-east4" => Ok(Self::UsEast4),
            "us-east5" => Ok(Self::UsEast5),
            "us-south1" => Ok(Self::UsSouth1),
            "us-west1" => Ok(Self::UsWest1),
            "us-west2" => Ok(Self::UsWest2),
            "us-west3" => Ok(Self::UsWest3),
            "us-west4" => Ok(Self::UsWest4),
            _ => Err(Error::InvalidDenoRegion),
        }
    }
}

impl TryFrom<&str> for DenoRegion {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Error> {
        s.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_region_name() {
        assert_eq!(DenoRegion::EuropeCentral2.name(), "europe-central2");
        assert_eq!(DenoRegion::UsSouth1.name(), "us-south1");
    }

    #[test]
    fn test_region_to_string() {
        assert_eq!(DenoRegion::EuropeCentral2.to_string(), "europe-central2");
        assert_eq!(DenoRegion::UsSouth1.to_string(), "us-south1");
    }

    #[test]
    fn test_region_from_str() {
        assert_eq!(DenoRegion::from_str("europe-central2"), Ok(DenoRegion::EuropeCentral2));
        assert_eq!("EUROPE-CENTRAL2".parse(), Ok(DenoRegion::EuropeCentral2));
        assert_eq!("europe-central2".try_into(), Ok(DenoRegion::EuropeCentral2));

        assert_eq!(DenoRegion::from_str("some-fake-region"), Err(Error::InvalidDenoRegion));
    }

    #[test]
    fn test_region_location() {
        let location = DenoRegion::AsiaNortheast2.location();
        assert_eq!(location, Location::new(34.7520, 135.4582));
        assert_eq!(location.latitude(), 34.7520);
        assert_eq!(location.longitude(), 135.4582);
    }

    #[test]
    fn test_region_distance_to() {
        let region = DenoRegion::EuropeNorth1;
        assert_eq!(region.distance_to(&region.location()), 0.0);
        assert_eq!(region.distance_to(&AwsRegion::EuWest2.location()), 1_840_385.671);
        assert_eq!(region.distance_to(&AwsRegion::EuWest3.location()), 1_886_010.429);
    }

    #[test]
    fn test_region_iter() {
        assert_eq!(DenoRegion::iter().next().unwrap().name(), "asia-east1");
        assert_eq!(DenoRegion::iter().last().unwrap().name(), "us-west4");
        assert_eq!(DenoRegion::iter().count(), 34);
    }

    #[test]
    fn test_to_aws_region() {
        let region: AwsRegion = DenoRegion::AsiaEast1.into();
        assert_eq!(region, AwsRegion::ApEast1);

        assert_eq!(AwsRegion::from(DenoRegion::AsiaEast1), AwsRegion::ApEast1);

        assert_eq!(Into::<AwsRegion>::into(DenoRegion::AsiaEast1), AwsRegion::ApEast1);
    }

    #[test]
    fn test_find_region_from_list() {
        struct Test {
            deno_region: DenoRegion,
            list: Vec<AwsRegion>,
            aws_region: AwsRegion,
        }

        let tests = vec![
            Test {
                deno_region: DenoRegion::EuropeWest3,
                list: vec![AwsRegion::EuWest1, AwsRegion::EuCentral1, AwsRegion::EuSouth1],
                aws_region: AwsRegion::EuCentral1,
            },
            Test {
                deno_region: DenoRegion::EuropeWest2,
                list: vec![AwsRegion::UsEast1, AwsRegion::UsWest1],
                aws_region: AwsRegion::UsEast1,
            },
            Test {
                deno_region: DenoRegion::UsWest4,
                list: vec![AwsRegion::UsGovEast1, AwsRegion::UsGovWest1],
                aws_region: AwsRegion::UsGovWest1,
            },
            Test {
                deno_region: DenoRegion::UsEast4,
                list: vec![AwsRegion::CaCentral1, AwsRegion::CaCentral1],
                aws_region: AwsRegion::CaCentral1,
            },
            Test {
                deno_region: DenoRegion::AsiaNortheast1,
                list: vec![AwsRegion::ApEast1, AwsRegion::ApNortheast3],
                aws_region: AwsRegion::ApNortheast3,
            },
        ];

        for t in tests {
            assert_eq!(t.deno_region.find_region_from_list(&t.list), t.aws_region);
        }
    }
}
