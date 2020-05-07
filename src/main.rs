use haversiner::{haversine, Point};

fn main() {
    let origin: Point = Point {
        lat: 36.12,
        lon: -86.67,
    };
    let destination: Point = Point {
        lat: 33.94,
        lon: -118.4,
    };
    let d: f64 = haversine(origin, destination);
    println!("Distance: {} km ({} mi)", d, d / 1.609_344);
}
