use std::f64;

static R: f64 = 6372.8;

pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

pub fn haversine(mut origin: Point, mut destination: Point) -> f64 {
    origin.lon -= destination.lon;
    origin.lon = origin.lon.to_radians();
    origin.lat = origin.lat.to_radians();
    destination.lat = destination.lat.to_radians();
    let dz: f64 = origin.lat.sin() - destination.lat.sin();
    let dx: f64 = origin.lon.cos() * origin.lat.cos() - destination.lat.cos();
    let dy: f64 = origin.lon.sin() * origin.lat.cos();
    ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R
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

    #[test]
    fn it_works_again() {
        let origin: Point = Point {
            lat: 39.472978,
            lon: -0.375203,
        };
        let destination: Point = Point {
            lat: 38.967161,
            lon: -0.184759,
        };

        assert_eq!(haversine(origin, destination), 58.604658305034434);
    }
}
