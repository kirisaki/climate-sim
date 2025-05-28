use bevy::prelude::*;
use bevy_pancam::{self, PanCam};
use geo_types::Coord;
use h3ron::{H3Cell, ToCoordinate};
use image::ImageReader;
use std::f32::consts::PI;
use crate::utils;

pub fn setup(
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
    let elevation_map = utils::bitmap2h3cell::h3_cells_to_elevation_map(&img, cells.into()).unwrap();

    // offset
    let center_point = utils::h3_to_local_coordinates(&center_cell, hex_scaling_factor);

    // generate hexmap
    for (cell, elev) in &elevation_map {
        // convert from h3
        let position = utils::h3_to_local_coordinates(&cell, hex_scaling_factor);

        // calcurate relatuve position
        let x = position.0 - center_point.0;
        let y = position.1 - center_point.1;

        // calculate hex radius
        let hex_radius = 32.5;

        // set colors
        let color = utils::elevation_to_color(*elev);

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
