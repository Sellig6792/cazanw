//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

extern crate cazanw;

wasm_bindgen_test_configure!(run_in_browser);

mod geometry {
    use super::wasm_bindgen_test;
    use cazanw::geometry::*;

    #[wasm_bindgen_test]
    fn test_distance() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 3, y: 4 };
        assert_eq!(distance(a, b), 5);
    }
}

mod triangulation {
    use super::wasm_bindgen_test;
    use assert_unordered::assert_eq_unordered;
    use cazanw::geometry::*;
    use cazanw::triangulation::*;

    #[wasm_bindgen_test]
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
                Point { x: 2, y: 2 }
            )
        );
        assert_eq!(
            triangles[1],
            Triangle(
                Point { x: 0, y: 0 },
                Point { x: 3, y: 4 },
                Point { x: 2, y: 2 }
            )
        );
    }

    #[wasm_bindgen_test]
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

    #[wasm_bindgen_test]
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

    #[wasm_bindgen_test]
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
                Point { x: 4, y: 0 }
            )
        );
    }
}
