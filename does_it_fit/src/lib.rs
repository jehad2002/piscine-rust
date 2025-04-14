pub mod areas_volumes;

use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let total_area = x * y;
    let required_area = match kind {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
    };

    total_area as f64 >= required_area * times as f64
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let total_volume = x * y * z;
    let required_volume = match kind {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => {
            areas_volumes::triangular_pyramid_volume(a as f64, b)
        }
        GeometricalVolumes::Parallelepiped => {
            areas_volumes::parallelepiped_volume(a, b, c) as f64
        }
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
    };

    total_volume as f64 >= required_volume * times as f64
}
