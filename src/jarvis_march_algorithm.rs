use std::cmp::Ordering;

use rand::{distributions::Uniform, prelude::*};

#[derive(Clone, Copy, Default, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Orientation {
    Collinear,
    Clockwise,
    CounterClockwise,
}

impl Orientation {
    fn from_points(p1: Point, p2: Point, p3: Point) -> Self {
        let v = (p2.y - p1.y) * (p3.x - p2.x) - (p2.x - p1.x) * (p3.y - p2.y);
        match v.cmp(&0) {
            Ordering::Less => Self::CounterClockwise,
            Ordering::Equal => Self::Collinear,
            Ordering::Greater => Self::Clockwise,
        }
    }
}

pub fn random_points(n: usize, width: isize, height: isize, margin: isize) -> Vec<Point> {
    let mut rng = rand::thread_rng();

    let x_low = -(width / 2) + margin;
    let x_high = (width / 2) - margin;
    let y_low = -(height / 2) + margin;
    let y_high = (height / 2) - margin;
    let x_dist = Uniform::new_inclusive(x_low, x_high);
    let y_dist = Uniform::new_inclusive(y_low, y_high);

    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let x = x_dist.sample(&mut rng);
        let y = y_dist.sample(&mut rng);
        points.push(Point::new(x, y));
    }
    points
}

pub fn find_convex_hull(points: &[Point]) -> Option<Vec<usize>> {
    if points.len() < 3 {
        return None;
    }

    let start = points
        .iter()
        .enumerate()
        .min_by_key(|(_, p)| p.x)
        .unwrap()
        .0;

    let mut hull = vec![start];
    loop {
        let mut candidate = (hull.last().unwrap() + 1) % points.len();
        for i in 0..points.len() {
            let p1 = points[*hull.last().unwrap()];
            let p2 = points[candidate];
            let p3 = points[i];
            if Orientation::from_points(p1, p2, p3) == Orientation::CounterClockwise {
                candidate = i;
            }
        }

        hull.push(candidate);
        if candidate == hull[0] {
            break;
        }
    }

    Some(hull)
}
