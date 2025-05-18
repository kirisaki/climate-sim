use bevy::prelude::*;
use bevy_pancam::{self, PanCam, PanCamPlugin};
use geo_types::Coord;
use h3ron::{H3Cell, Index, ToCoordinate};
use image::ImageReader;
use std::f32::consts::PI;
mod bitmap2h3cell;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // rows
    r: i32, // columns
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let img = ImageReader::open("test.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();
    commands.spawn((Camera2d, PanCam::default()));

    // params for h3
    let resolution = 5;
    let center_lat = 35.6895;
    let center_lon = 139.6917;
    let center_cell = H3Cell::from_coordinate(
        Coord {
            y: center_lat,
            x: center_lon,
        },
        resolution,
    )
    .unwrap();
    let k_distance = 30;
    // params
    let cells = center_cell.grid_disk(k_distance).unwrap();
    let hex_scaling_factor = 400.0;
    let elevation_map = bitmap2h3cell::h3_cells_to_elevation_map(&img, cells.into()).unwrap();

    // offset
    let center_point = h3_to_local_coordinates(&center_cell, hex_scaling_factor);

    // generate hexmap
    for (cell, elev) in &elevation_map {
        // convert from h3
        let position = h3_to_local_coordinates(&cell, hex_scaling_factor);

        // calcurate relatuve position
        let x = position.0 - center_point.0;
        let y = position.1 - center_point.1;

        // calculate hex radius
        let hex_radius = 32.5;

        // set colors
        let color = elevation_to_color(*elev);

        // generate hexagonal mesh
        let hex_mesh = meshes.add(RegularPolygon::new(hex_radius, 6));

        // generate hex
        commands.spawn((
            MeshMaterial2d(materials.add(color)),
            Mesh2d(hex_mesh),
            Transform::from_xyz(x, y, 0.0).with_rotation(Quat::from_rotation_z(PI / 12.0)),
        ));

        // display hex infomation
        let co = cell.to_coordinate().unwrap();
        let info_text = format!("lat: {:.3}\nlng:{:.3}\nelev: {:.1}", co.y, co.x, elev);
        commands.spawn((
            Text2d::new(info_text),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            Transform::from_xyz(x, y, 1.0),
        ));
    }
}

fn h3_to_local_coordinates(cell: &H3Cell, scaling_factor: f32) -> (f32, f32) {
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
