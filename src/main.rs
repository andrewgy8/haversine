use std::f64;

static R: f64 = 6372.8;

struct Point {
    lat: f64,
    lon: f64,
}

fn haversine(mut origin: Point, mut destination: Point) -> f64 {
    origin.lon -= destination.lon;
    origin.lon = origin.lon.to_radians();
    origin.lat = origin.lat.to_radians();
    destination.lat = destination.lat.to_radians();
    let dz: f64 = origin.lat.sin() - destination.lat.sin();
    let dx: f64 = origin.lon.cos() * origin.lat.cos() - destination.lat.cos();
    let dy: f64 = origin.lon.sin() * origin.lat.cos();
    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
}

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
    println!("Distance: {} km ({} mi)", d, d / 1.609344);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let origin: Point = Point {
            lat: 36.12,
            lon: -86.67,
        };
        let destination: Point = Point {
            lat: 33.94,
            lon: -118.4,
        };

        assert_eq!(haversine(origin, destination), 2887.2599506071106);
    }
}
