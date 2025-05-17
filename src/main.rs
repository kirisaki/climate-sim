use bevy::prelude::*;
use bevy_pancam::{self, PanCam, PanCamPlugin};
use geo_types::Coord;
use h3ron::{H3Cell, Index, ToCoordinate};
use std::f32::consts::PI;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // rows
    r: i32, // columns
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, PanCamPlugin::default()))
        .add_systems(Startup, setup);
    #[cfg(not(target_arch = "wasm32"))]
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
    let k_distance = match resolution {
        4 => 7,
        5 => 20,
        6 => 30,
        _ => 15,
    };
    // params
    let cells = center_cell.grid_disk(k_distance).unwrap();
    let hex_scaling_factor = match resolution {
        4 => 800.0,
        5 => 400.0,
        6 => 200.0,
        _ => 400.0,
    };

    // offset
    let center_point = h3_to_local_coordinates(&center_cell, hex_scaling_factor);

    // generate hexmap
    for cell in &cells {
        // convert from h3
        let position = h3_to_local_coordinates(&cell, hex_scaling_factor);

        // calcurate relatuve position
        let x = position.0 - center_point.0;
        let y = position.1 - center_point.1;

        // calculate hex radius
        let hex_radius = match resolution {
            4 => 45.0,
            5 => 32.5,
            6 => 15.0,
            _ => 25.0,
        };

        // set colors
        let hue = (Vec2::new(x, y).to_angle() + PI) / (2.0 * PI) * 360.0;
        let distance_from_center = (x * x + y * y).sqrt() / (hex_scaling_factor * 0.2);
        let saturation = 1.0 - 0.8 * (distance_from_center / k_distance as f32).min(1.0);
        let lightness = 0.5;
        let color = Color::hsl(hue, saturation, lightness);

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
        let info_text = format!("lat: {:.3}\nlng:{:.3}\nsat: {:.3}", co.y, co.x, saturation);
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
    // get center position of h3 cell
    let co = cell.to_coordinate().unwrap();

    // convert the coordinate to position
    let x = co.x as f32 * scaling_factor;
    let y = co.y as f32 * scaling_factor;

    (x, y)
}
