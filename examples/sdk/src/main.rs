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
    use aws_types::region::Region;

    let region = aws_region_nearby::find_region(latitude, longitude);
    let sdk_region = Region::from_static(region.name());

    println!("AWS SDK region = {:?}", sdk_region);
}

fn rusoto_example(latitude: f64, longitude: f64) {
    use rusoto_core::Region;
    use std::str::FromStr;

    let region = aws_region_nearby::find_region(latitude, longitude);
    let rusoto_region = Region::from_str(region.name()).unwrap();

    println!("Rusoto region = {:?}", rusoto_region);
}
