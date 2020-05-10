use std::f64;

static R: f64 = 6372.8;

#[derive(Copy, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

pub struct Measure {
    pub distance: f64,
}

#[derive(Default)]
pub struct Matrix {
    pub distances: Vec<f64>,
}

impl Matrix {
    pub fn add_distance(&mut self, measure: Measure) {
        self.distances.push(measure.distance);
    }
}

pub fn haversine(mut origin: Point, mut destination: Point) -> Measure {
    origin.lon -= destination.lon;
    origin.lon = origin.lon.to_radians();
    origin.lat = origin.lat.to_radians();
    destination.lat = destination.lat.to_radians();
    let dz: f64 = origin.lat.sin() - destination.lat.sin();
    let dx: f64 = origin.lon.cos() * origin.lat.cos() - destination.lat.cos();
    let dy: f64 = origin.lon.sin() * origin.lat.cos();
    let dist: f64 = ((dx * dx + dy * dy + dz * dz).sqrt() / 2.0).asin() * 2.0 * R;
    Measure { distance: dist }
}

pub fn one_to_many(origin: Point, destinations: Vec<Point>) -> Matrix {
    let mut matrix: Matrix = Matrix::default();
    let origin_distance: Measure = haversine(origin, origin);
    matrix.add_distance(origin_distance);
    for destination in destinations {
        let measure = haversine(origin, destination);
        matrix.add_distance(measure);
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_one_point() {
        let origin: Point = Point {
            lat: 36.12,
            lon: -86.67,
        };
        let destination: Point = Point {
            lat: 33.94,
            lon: -118.4,
        };
        let measure: Measure = haversine(origin, destination);
        assert_eq!(measure.distance, 2887.2599506071106);
    }

    #[test]
    fn it_works_for_one_point_again() {
        let origin: Point = Point {
            lat: 39.472978,
            lon: -0.375203,
        };
        let destination: Point = Point {
            lat: 38.967161,
            lon: -0.184759,
        };

        let measure = haversine(origin, destination);
        assert_eq!(measure.distance, 58.604658305034434);
    }

    #[test]
    fn it_works_for_one_point_to_many() {
        let origin: Point = Point {
            lat: 39.472978,
            lon: -0.375203,
        };
        let mut destinations: Vec<Point> = Vec::new();
        destinations.push(Point {
            lat: 38.967161,
            lon: -0.184759,
        });
        destinations.push(Point {
            lat: 33.94,
            lon: -118.4,
        });

        let matrix = one_to_many(origin, destinations);
        assert_eq!(
            matrix.distances,
            [0.0, 58.604658305034434, 9665.831848220572]
        );
    }
}
