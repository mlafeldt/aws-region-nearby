use strum::{Display, EnumIter, EnumProperty, EnumString};

// Region data taken from https://gist.github.com/tobilg/ba6a5e1635478d13efdea5c1cd8227de
#[derive(Debug, PartialEq, Display, EnumIter, EnumProperty, EnumString)]
pub enum AwsRegion {
    #[strum(
        serialize = "af-south-1",
        props(
            city = "Cape Town",
            country = "South Africa",
            latitude = "-33.9648017883",
            longitude = "18.6016998291"
        )
    )]
    AfSouth1,

    #[strum(
        serialize = "ap-east-1",
        props(
            city = "Hong Kong",
            country = "China",
            latitude = "22.308901",
            longitude = "113.915001"
        )
    )]
    ApEast1,

    #[strum(
        serialize = "ap-northeast-1",
        props(
            city = "Tokyo",
            country = "Japan",
            latitude = "35.764702",
            longitude = "140.386002"
        )
    )]
    ApNortheast1,

    #[strum(
        serialize = "ap-northeast-2",
        props(
            city = "Seoul",
            country = "South Korea",
            latitude = "37.46910095214844",
            longitude = "126.45099639892578"
        )
    )]
    ApNortheast2,

    #[strum(
        serialize = "ap-northeast-3",
        props(
            city = "Osaka",
            country = "Japan",
            latitude = "34.42729949951172",
            longitude = "135.24400329589844"
        )
    )]
    ApNortheast3,

    #[strum(
        serialize = "ap-south-1",
        props(
            city = "Mumbai",
            country = "India",
            latitude = "19.0886993408",
            longitude = "72.8678970337"
        )
    )]
    ApSouth1,

    #[strum(
        serialize = "ap-southeast-1",
        props(
            city = "Singapore",
            country = "Singapore",
            latitude = "1.35019",
            longitude = "103.994003"
        )
    )]
    ApSoutheast1,

    #[strum(
        serialize = "ap-southeast-2",
        props(
            city = "Sydney",
            country = "Australia",
            latitude = "-33.94609832763672",
            longitude = "151.177001953125"
        )
    )]
    ApSoutheast2,

    #[strum(
        serialize = "ap-southeast-3",
        props(
            city = "Jakarta",
            country = "Indonesia",
            latitude = "-6.125556",
            longitude = "106.655833"
        )
    )]
    ApSoutheast3,

    #[strum(
        serialize = "ca-central-1",
        props(
            city = "Montreal",
            country = "Canada",
            latitude = "45.470556",
            longitude = "-73.740833"
        )
    )]
    CaCentral1,

    #[strum(
        serialize = "cn-north-1",
        props(
            city = "Beijing",
            country = "China",
            latitude = "40.080101013183594",
            longitude = "116.58499908447266"
        )
    )]
    CnNorth1,

    #[strum(
        serialize = "cn-northwest-1",
        props(
            city = "Yinchuan",
            country = "China",
            latitude = "38.321667",
            longitude = "106.3925"
        )
    )]
    CnNorthwest1,

    #[strum(
        serialize = "eu-central-1",
        props(
            city = "Frankfurt",
            country = "Germany",
            latitude = "50.033333",
            longitude = "8.570556"
        )
    )]
    EuCentral1,

    #[strum(
        serialize = "eu-north-1",
        props(
            city = "Stockholm",
            country = "Sweden",
            latitude = "59.651901245117",
            longitude = "17.918600082397"
        )
    )]
    EuNorth1,

    #[strum(
        serialize = "eu-south-1",
        props(
            city = "Milan",
            country = "Italy",
            latitude = "45.6306",
            longitude = "8.72811"
        )
    )]
    EuSouth1,

    #[strum(
        serialize = "eu-west-1",
        props(
            city = "Dublin",
            country = "Ireland",
            latitude = "53.421299",
            longitude = "-6.27007"
        )
    )]
    EuWest1,

    #[strum(
        serialize = "eu-west-2",
        props(
            city = "London",
            country = "England",
            latitude = "51.4775",
            longitude = "-0.461389"
        )
    )]
    EuWest2,

    #[strum(
        serialize = "eu-west-3",
        props(
            city = "Paris",
            country = "France",
            latitude = "49.012798",
            longitude = "2.55"
        )
    )]
    EuWest3,

    #[strum(
        serialize = "me-south-1",
        props(
            city = "Manama",
            country = "Bahrain",
            latitude = "26.27079963684082",
            longitude = "50.63359832763672"
        )
    )]
    MeSouth1,

    #[strum(
        serialize = "sa-east-1",
        props(
            city = "Sao Paulo",
            country = "Brazil",
            latitude = "-23.435556",
            longitude = "-46.473056"
        )
    )]
    SaEast1,

    #[strum(
        serialize = "us-east-1",
        props(
            city = "Ashburn, Virginia",
            country = "United States",
            latitude = "38.9445",
            longitude = "-77.4558029"
        )
    )]
    UsEast1,

    #[strum(
        serialize = "us-east-2",
        props(
            city = "Columbus, Ohio",
            country = "United States",
            latitude = "39.958993960575775",
            longitude = "-83.00219086148725"
        )
    )]
    UsEast2,

    #[strum(
        serialize = "us-west-1",
        props(
            city = "San Francisco, California",
            country = "United States",
            latitude = "37.61899948120117",
            longitude = "-122.375"
        )
    )]
    UsWest1,

    #[strum(
        serialize = "us-west-2",
        props(
            city = "Hillsboro, Oregon",
            country = "United States",
            latitude = "45.540394",
            longitude = "-122.949825"
        )
    )]
    UsWest2,

    #[strum(
        serialize = "us-gov-east-1",
        props(
            city = "Ashburn, Virginia",
            country = "United States",
            latitude = "38.9445",
            longitude = "-77.4558029"
        )
    )]
    UsGovEast1,

    #[strum(
        serialize = "us-gov-west-1",
        props(
            city = "San Francisco, California",
            country = "United States",
            latitude = "37.61899948120117",
            longitude = "-122.375"
        )
    )]
    UsGovWest1,
}
