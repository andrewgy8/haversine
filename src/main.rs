use haversiner::{haversine, Measure, Point};

fn main() {
    let origin: Point = Point {
        lat: 36.12,
        lon: -86.67,
    };
    let destination: Point = Point {
        lat: 33.94,
        lon: -118.4,
    };
    let measure: Measure = haversine(origin, destination);
    println!(
        "Distance: {} km ({} mi)",
        measure.distance,
        measure.distance / 1.609_344
    );
}
