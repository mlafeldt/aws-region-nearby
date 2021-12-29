use geoutils::Location;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AwsRegion {
    AfSouth1,
    ApEast1,
    ApNortheast1,
    ApNortheast2,
    ApNortheast3,
    ApSouth1,
    ApSoutheast1,
    ApSoutheast2,
    ApSoutheast3,
    CaCentral1,
    CnNorth1,
    CnNorthwest1,
    EuCentral1,
    EuNorth1,
    EuSouth1,
    EuWest1,
    EuWest2,
    EuWest3,
    MeSouth1,
    SaEast1,
    UsEast1,
    UsEast2,
    UsWest1,
    UsWest2,
    UsGovEast1,
    UsGovWest1,
}

impl AwsRegion {
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
            AwsRegion::SaEast1 => Location::new(-23.435556, -46.473056),         // Sao Paulo, Brazil
            AwsRegion::UsEast1 => Location::new(38.9445, -77.4558029),           // Ashburn, Virginia, USA
            AwsRegion::UsEast2 => Location::new(39.958993960575775, -83.00219086148725), // Columbus, Ohio, USA
            AwsRegion::UsWest1 => Location::new(37.61899948120117, -122.375),    // San Francisco, California, USA
            AwsRegion::UsWest2 => Location::new(45.540394, -122.949825),         // Hillsboro, Oregon, USA
            AwsRegion::UsGovEast1 => Location::new(38.9445, -77.4558029),        // Ashburn, Virginia, USA
            AwsRegion::UsGovWest1 => Location::new(37.61899948120117, -122.375), // San Francisco, California, USA
        }
    }

    pub fn distance_to(&self, to: &Location) -> f64 {
        self.location().haversine_distance_to(to).meters()
    }
}
