use bevy::prelude::*;
use h3ron::{H3Cell, ToCoordinate};
pub mod bitmap2h3cell;

pub fn h3_to_local_coordinates(cell: &H3Cell, scaling_factor: f32) -> (f32, f32) {
    // get center position of h2 cell
    let co = cell.to_coordinate().unwrap();

    // convert the coordinate to position
    let x = co.x as f32 * scaling_factor;
    let y = co.y as f32 * scaling_factor;

    (x, y)
}

pub fn elevation_to_color(elevation: f64) -> Color {
    if elevation <= 0.0 {
        // waters: normalize -1000〜0 m into 0.0〜1.0
        let t = (elevation / -1000.0).clamp(0.0, 1.0) * (-1.0) + 1.0;

        // HSL: cyan to navy
        let hue = 220.0 - t * 20.0; // 220 → 200
        let sat = 0.6 - t * 0.1; // 0.6 → 0.5
        let light = 0.3 + t * 0.4; // 0.3 → 0.7

        Color::hsl(hue as f32, sat as f32, light as f32)
    } else {
        // ground: normalize 0〜8000 m into 0.0〜1.0
        let t = (elevation / 8000.0).clamp(0.0, 1.0);

        // HSL: 茶 → 白
        let hue = 30.0 * (1.0 - t); // 30 → 0
        let sat = 0.5 * (1.0 - t); // 0.5 → 0
        let light = 0.3 + t * 0.7; // 0.3 → 1.0

        Color::hsl(hue as f32, sat as f32, light as f32)
    }
}
