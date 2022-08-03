use rand::Rng;

// Jetzt gehen wir auf U-Boot-Jagd.
//
// 3 Sonar-Bojen befinden sich an den Koordinaten: B1(2|1); B2(9|4); B3(5|7).
// Die Laufzeit des Schalls zu dem gesuchten U-Boot ist von allen Bojen gleich lang.
//
// An welcher Koordinate befindet sich das gesuchte Objekt?
//
// - source: https://mathe.dlewand.de/

const MAX_X: f32 = 10.0;
const MAX_Y: f32 = 10.0;
const ITERATIONS: u32 = 1000000;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32
}

impl Point {
    fn random() -> Point {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0.0..MAX_X),
            y: rng.gen_range(0.0..MAX_Y)
        }
    }

    fn distance_to(&self, point: Point) -> f32 {
        ((point.x-self.x).powf(2.0) + (point.y-self.y).powf(2.0)).sqrt() 
    }
}

fn main() {
    const B1: Point = Point { x: 2.0, y: 1.0 };
    const B2: Point = Point { x: 9.0, y: 4.0 };
    const B3: Point = Point { x: 5.0, y: 7.0 };

    let mut best_mean_d = MAX_X;
    for i in 0..ITERATIONS {
        let p = Point::random();
        let d1 = B1.distance_to(p);
        let d2 = B2.distance_to(p);
        let d3 = B3.distance_to(p);
        let mean_d = ((d1-d2).abs() + (d1-d3).abs() + (d2-d3).abs()) / 3.0;
        if mean_d < best_mean_d {
            best_mean_d = mean_d;
            println!("i={} x={:6.4} y={:6.4} d={:6.4}", i, p.x, p.y, best_mean_d);
        }
    }
}

