use std::cmp::Ordering;
use num_integer::Roots;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }
}

pub type Polygon = Vec<Point>;

#[wasm_bindgen]
pub fn distance(a: Point, b: Point) -> u16 {
    let dx = (a.x as i32 - b.x as i32).abs() as u16;
    let dy = (a.y as i32 - b.y as i32).abs() as u16;
    (dx * dx + dy * dy).sqrt()
}

pub(crate) fn cross_product(a: Point, b: Point, c: Point) -> i32 {
    (b.x as i32 - a.x as i32) * (c.y as i32 - b.y as i32) - (b.y as i32 - a.y as i32) * (c.x as i32 - b.x as i32)
}

pub(crate) fn is_point_inside_triangle(triangle: &Polygon, point: Point) -> bool {
    let a = triangle[0];
    let b = triangle[1];
    let c = triangle[2];

    match (
        cross_product(a, b, point).cmp(&0),
        cross_product(b, c, point).cmp(&0),
        cross_product(c, a, point).cmp(&0),
    ) {
        (Ordering::Less, Ordering::Less, Ordering::Less) => true,
        (Ordering::Greater, Ordering::Greater, Ordering::Greater) => true,
        _ => false,
    }
}



pub(crate) fn is_convex(polygon: &Polygon) -> bool {
    let mut sign = 0;
    for i in 0..polygon.len() {
        let a = polygon[i];
        let b = polygon[(i + 1) % polygon.len()];
        let c = polygon[(i + 2) % polygon.len()];
        let cross = cross_product(a, b, c);
        if cross == 0 {
            continue;
        }
        if sign == 0 {
            sign = cross;
        } else if sign * cross < 0 {
            return false;
        }
    }
    true
}
