pub mod areas_volumes;

use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    container_dimensions: (u32, u32),
    shape: GeometricalShapes,
    count: u32,
    shape_dimensions: (u32, u32),
) -> bool {
    let container_area = container_dimensions.0 * container_dimensions.1;
    let shape_area = match shape {
        GeometricalShapes::Square => shape_dimensions.0.pow(2),
        GeometricalShapes::Rectangle => shape_dimensions.0 * shape_dimensions.1,
        GeometricalShapes::Triangle => (shape_dimensions.0 * shape_dimensions.1) / 2,
        GeometricalShapes::Circle => {
            let r = shape_dimensions.0;
            (std::f64::consts::PI * (r.pow(2) as f64)) as u32
        }
    };

    count * shape_area <= container_area
}

pub fn volume_fit(
    container_dimensions: (u32, u32, u32),
    volume: GeometricalVolumes,
    count: u32,
    shape_dimensions: (u32, u32, u32),
) -> bool {
    let container_volume = container_dimensions.0 * container_dimensions.1 * container_dimensions.2;
    let shape_volume = match volume {
        GeometricalVolumes::Cube => shape_dimensions.0.pow(3),
        GeometricalVolumes::Parallelepiped => {
            shape_dimensions.0 * shape_dimensions.1 * shape_dimensions.2
        }
        GeometricalVolumes::Sphere => {
            let r = shape_dimensions.0;
            ((4.0 / 3.0) * std::f64::consts::PI * (r.pow(3) as f64)) as u32
        }
        GeometricalVolumes::Cylinder => {
            let r = shape_dimensions.0;
            let h = shape_dimensions.1;
            (std::f64::consts::PI * (r.pow(2) as f64) * (h as f64)) as u32
        }
    };

    count * shape_volume <= container_volume
}
