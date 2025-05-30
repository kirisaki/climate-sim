use crate::utils;
use bevy::prelude::*;
use bevy_pancam::{self, PanCam};
use geo_types::Coord;
use h3ron::H3Cell;
use image::ImageReader;

#[derive(Resource)]
pub struct ElevationData {
    pub elevation_map: Vec<(H3Cell, f64)>,
    pub center_point: (f32, f32),
    pub hex_scaling_factor: f32,
    pub hex_radius: f32,
}

#[derive(Component)]
pub struct HexCell {
    pub cell: H3Cell,
    pub elevation: f64,
}

pub fn setup(mut commands: Commands) {
    // Load image data
    let img = ImageReader::open("test.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();

    // Setup camera
    commands.spawn((Camera2d, PanCam::default()));

    // Setup H3 parameters
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
    let hex_scaling_factor = 400.0;
    let hex_radius = 32.5;

    // Generate cells
    let cells = center_cell.grid_disk(k_distance).unwrap();
    let elevation_map =
        utils::bitmap2h3cell::h3_cells_to_elevation_map(&img, cells.into()).unwrap();

    // Calculate center point
    let center_point = utils::h3_to_local_coordinates(&center_cell, hex_scaling_factor);

    // Create entity for each cell (rendering components will be added later)
    for (cell, elev) in &elevation_map {
        let position = utils::h3_to_local_coordinates(&cell, hex_scaling_factor);
        let x = position.0 - center_point.0;
        let y = position.1 - center_point.1;

        commands.spawn((
            HexCell {
                cell: cell.clone(),
                elevation: *elev,
            },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    // Store elevation data as resource
    commands.insert_resource(ElevationData {
        elevation_map,
        center_point,
        hex_scaling_factor,
        hex_radius,
    });
}
