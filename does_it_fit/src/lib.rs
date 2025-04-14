pub mod areas_volumes;

use areas_volumes::*;

pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    // حساب المساحة المطلوبة حسب نوع الشكل
    let area_needed = match kind {
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };

    // حساب المساحة المتاحة
    let total_area = (x * y) as f64;

    // هل المساحة المطلوبة مضروبة في عدد التكرارات أقل أو تساوي المتاحة؟
    area_needed * times as f64 <= total_area
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    // حساب الحجم المطلوب حسب نوع المجسم
    let volume_needed = match kind {
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
    };

    // حساب الحجم المتاح
    let total_volume = (x * y * z) as f64;

    // هل الحجم المطلوب مضروب في عدد التكرارات أقل أو يساوي الحجم المتاح؟
    volume_needed * times as f64 <= total_volume
}