use crate::geometry::*;
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) fn is_ear(polygon: &Polygon, i: usize) -> bool {
    let len = polygon.len();

    // Get the points for the triangle
    let prev = &polygon[(i + len - 1) % len];
    let curr = &polygon[i];
    let next = &polygon[(i + 1) % len];

    // Check if the triangle is counter-clockwise
    if !is_counter_clockwise(prev, curr, next) {
        return false;
    }

    // Check if any points are inside the triangle
    for point in polygon.iter() {
        if point == prev || point == curr || point == next {
            continue;
        }
        if is_point_inside_triangle(&Triangle(*prev, *curr, *next), *point) {
            return false;
        }
    }

    true
}

// Checks if the points form a counter-clockwise turn
fn is_counter_clockwise(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x as i32 - a.x as i32) * (c.y as i32 - a.y as i32)
        > (b.y as i32 - a.y as i32) * (c.x as i32 - b.x as i32)
}

#[wasm_bindgen]
pub fn triangulate(polygon: Polygon) -> Option<Vec<Triangle>> {
    if polygon.len() == 3 {
        return Some(vec![Triangle(polygon[0], polygon[1], polygon[2])]);
    }
    if is_convex(&polygon) {
        return None;
    }

    for i in 0..polygon.len() {
        let prev = if i == 0 { polygon.len() - 1 } else { i - 1 };
        let next = if i == polygon.len() - 1 { 0 } else { i + 1 };

        let triangle = Triangle(polygon[prev], polygon[i], polygon[next]);

        if is_ear(&polygon, i) {
            let mut new_polygon = polygon.clone();
            new_polygon.remove(i);

            if let Some(mut triangles) = triangulate(new_polygon) {
                triangles.push(triangle);
                return Some(triangles);
            }
        }
    }

    None
}
