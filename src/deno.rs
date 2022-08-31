use std::fmt;
use std::str::FromStr;

use crate::aws::AwsRegion;
use crate::Error;

/// A Deno Deploy region. Based on <https://deno.com/deploy/docs/regions>
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        const REGIONS: [DenoRegion; 34] = [
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

impl From<DenoRegion> for AwsRegion {
    fn from(region: DenoRegion) -> Self {
        match region {
            DenoRegion::AsiaEast1 => Self::ApEast1, // assuming Taipei City
            DenoRegion::AsiaEast2 => Self::ApEast1,
            DenoRegion::AsiaNortheast1 => Self::ApNortheast1,
            DenoRegion::AsiaNortheast2 => Self::ApNortheast3,
            DenoRegion::AsiaNortheast3 => Self::ApNortheast2,
            DenoRegion::AsiaSouth1 => Self::ApSouth1,
            DenoRegion::AsiaSouth2 => Self::ApSouth1,
            DenoRegion::AsiaSoutheast1 => Self::ApSoutheast1,
            DenoRegion::AsiaSoutheast2 => Self::ApSoutheast3,
            DenoRegion::AustraliaSoutheast1 => Self::ApSoutheast2,
            DenoRegion::AustraliaSoutheast2 => Self::ApSoutheast2,
            DenoRegion::EuropeCentral2 => Self::EuNorth1,
            DenoRegion::EuropeNorth1 => Self::EuNorth1, // assuming Helsinki
            DenoRegion::EuropeWest1 => Self::EuWest3,   // assuming Brussels
            DenoRegion::EuropeWest2 => Self::EuWest2,
            DenoRegion::EuropeWest3 => Self::EuCentral1,
            DenoRegion::EuropeWest4 => Self::EuCentral1, // assuming Amsterdam
            DenoRegion::EuropeWest6 => Self::EuSouth1,
            DenoRegion::EuropeWest8 => Self::EuSouth1,
            DenoRegion::EuropeWest9 => Self::EuWest3,
            DenoRegion::EuropeSouthwest1 => Self::EuSouth1,
            DenoRegion::NorthamericaNortheast1 => Self::CaCentral1,
            DenoRegion::NorthamericaNortheast2 => Self::CaCentral1,
            DenoRegion::SouthamericaEast1 => Self::SaEast1,
            DenoRegion::SouthamericaWest1 => Self::SaEast1, // assuming Santiago
            DenoRegion::UsCentral1 => Self::UsEast2,        // assuming Des Moines, IA
            DenoRegion::UsEast1 => Self::UsEast1,           // assuming Columbia, SC
            DenoRegion::UsEast4 => Self::UsEast1,
            DenoRegion::UsEast5 => Self::UsEast2,
            DenoRegion::UsSouth1 => Self::UsEast2, // assuming Austin, TX
            DenoRegion::UsWest1 => Self::UsWest2,
            DenoRegion::UsWest2 => Self::UsWest1,
            DenoRegion::UsWest3 => Self::UsWest1, // assuming Salt Lake City, UT
            DenoRegion::UsWest4 => Self::UsWest1, // assuming Carson City, NV
        }
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
}
