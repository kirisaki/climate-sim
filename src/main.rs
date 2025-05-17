use bevy::prelude::*;
use bevy_pancam::{self, PanCam, PanCamPlugin};

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

    // size of hexmap
    let width = 10;
    let height = 8;

    // params
    let hex_radius = 30.0;
    let hex_height = hex_radius * 2.0;
    let hex_width = hex_radius * 3.0_f32.sqrt();
    let hex_spacing_x = hex_width;
    let hex_spacing_y = hex_height * 0.75;

    // generate hexmap
    for r in 0..height {
        for q in 0..width {
            // calcurate offsets
            let offset_q = q as f32;
            let offset_r = r as f32;

            // shift the position if even line
            let x = offset_q * hex_spacing_x + if r % 2 == 1 { hex_spacing_x / 2.0 } else { 0.0 };
            let y = -offset_r * hex_spacing_y;

            // the color of the hex
            let color = Color::hsl(
                ((q as f32 / width as f32) * 360.0) % 360.0,
                0.5 + (r as f32 / height as f32) * 0.5,
                0.5,
            );

            // make hex mesh
            let hex_mesh = meshes.add(RegularPolygon {
                sides: 6,
                circumcircle: Circle { radius: hex_radius },
                ..default()
            });

            // make hex
            commands.spawn((
                Mesh2d(hex_mesh),
                MeshMaterial2d(materials.add(color)),
                Transform::from_xyz(x, y, 0.0),
                HexCoord { q, r },
            ));

            // add text to the hex
            commands.spawn((
                Text2d::new(format!("{},{}", q, r)),
                Transform::from_xyz(x, y, 1.0),
            ));
        }
    }
}
