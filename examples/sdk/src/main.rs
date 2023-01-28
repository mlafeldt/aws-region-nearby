// Usage:   cargo run <latitude> <longitude>
// Example: cargo run 40.714728 -73.998672

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let latitude = args[0].parse::<f64>().unwrap();
    let longitude = args[1].parse::<f64>().unwrap();

    aws_sdk_example(latitude, longitude);
    rusoto_example(latitude, longitude);
}

fn aws_sdk_example(latitude: f64, longitude: f64) {
    use aws_region_nearby::find_region;
    use aws_types::region::Region;

    let region = Region::from_static(find_region(latitude, longitude).name());

    println!("AWS SDK region = {region:?}");
}

fn rusoto_example(latitude: f64, longitude: f64) {
    use aws_region_nearby::{find_region_from_list, AwsRegion};

    let replica_regions: Vec<AwsRegion> = vec!["us-west-1", "us-east-1", "eu-central-1", "ap-northeast-1"]
        .iter()
        .map(|r| r.parse().unwrap())
        .collect();

    let region: rusoto_core::Region = find_region_from_list(latitude, longitude, &replica_regions)
        .name()
        .parse()
        .unwrap();

    println!("Rusoto region = {region:?}");
}
