use crate::geometry::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

pub fn is_ear(polygon: &Polygon, i: usize) -> bool {
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

#[cfg_attr(feature = "wasm", wasm_bindgen)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use assert_unordered::assert_eq_unordered;

    #[test]
    fn test_triangulate() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
            Point { x: 2, y: 2 },
        ];
        let triangles = match triangulate(polygon) {
            Some(triangles) => triangles,
            None => panic!(),
        };
        assert_eq!(triangles.len(), 2);
        assert_eq!(
            triangles[0],
            Triangle(
                Point { x: 3, y: 4 },
                Point { x: 4, y: 0 },
                Point { x: 2, y: 2 },
            )
        );
        assert_eq!(
            triangles[1],
            Triangle(
                Point { x: 0, y: 0 },
                Point { x: 3, y: 4 },
                Point { x: 2, y: 2 },
            )
        );
    }

    #[test]
    fn test_triangulate_n_vertices() {
        let (a, b, c, d, e, f, g, h) = (
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
            Point { x: 6, y: 1 },
            Point { x: 7, y: 0 },
            Point { x: 7, y: 3 },
            Point { x: 6, y: 5 },
            Point { x: 1, y: 8 },
        );
        let polygon = vec![a, b, c, d, e, f, g, h];
        let triangles = triangulate(polygon).unwrap();
        assert_eq!(triangles.len(), 6);

        let right_triangles = vec![
            Triangle(b, g, h),
            Triangle(b, c, g),
            Triangle(a, b, h),
            Triangle(c, g, f),
            Triangle(e, f, d),
            Triangle(c, f, d),
        ];

        assert_eq_unordered!(triangles, right_triangles);
    }

    #[test]
    fn test_triangulate_2() {
        // (2,8) (5,14) (6,16) sont sur la meme ligne --> pas de triangle --> supprimer les triangles qui sont plats
        let (a, b, c, d, e, f, g, h, i, j, k, l, m) = (
            Point { x: 0, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 2 },
            Point { x: 8, y: 1 },
            Point { x: 10, y: 6 },
            Point { x: 6, y: 4 },
            Point { x: 10, y: 8 },
            Point { x: 6, y: 10 },
            Point { x: 6, y: 16 },
            Point { x: 5, y: 14 },
            Point { x: 4, y: 15 },
            Point { x: 0, y: 16 },
            Point { x: 2, y: 8 },
        );

        let polygon = vec![a, b, c, d, e, f, g, h, i, j, k, l, m];

        let triangles = triangulate(polygon).unwrap();

        assert_eq!(triangles.len(), 11);
        assert_eq_unordered!(
            triangles,
            vec![
                Triangle(b, c, m),
                Triangle(c, d, m),
                Triangle(d, e, m),
                Triangle(f, g, h),
                Triangle(f, h, i),
                Triangle(f, i, j),
                Triangle(f, j, k),
                Triangle(f, k, l),
                Triangle(f, l, m),
                Triangle(f, m, a),
                Triangle(f, a, b),
            ]
        );
    }

    #[test]
    fn test_triangulate_convex() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 7, y: 3 },
            Point { x: 6, y: 1 },
            Point { x: 3, y: 0 },
        ];
        let triangles = triangulate(polygon);
        assert_eq!(triangles, None);
        // assert_eq!(is_convex(&polygon), true);
    }

    #[test]
    fn test_triangulate_triangle() {
        let polygon = vec![
            Point { x: 0, y: 0 },
            Point { x: 3, y: 4 },
            Point { x: 4, y: 0 },
        ];
        let triangles = match triangulate(polygon) {
            Some(triangles) => triangles,
            None => panic!(),
        };
        assert_eq!(triangles.len(), 1);
        assert_eq!(
            triangles[0],
            Triangle(
                Point { x: 0, y: 0 },
                Point { x: 3, y: 4 },
                Point { x: 4, y: 0 },
            )
        );
    }
}
