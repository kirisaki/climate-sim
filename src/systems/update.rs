use crate::systems::setup::{ElevationData, HexCell};
use crate::utils;
use bevy::prelude::*;
use h3ron::ToCoordinate;
use std::f32::consts::PI;

pub fn render_hex_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    elevation_data: Res<ElevationData>,
    query: Query<(Entity, &HexCell, &Transform), Without<Mesh2d>>,
) {
    for (entity, hex_cell, transform) in query.iter() {
        let color = utils::elevation_to_color(hex_cell.elevation);

        // Generate hexagonal mesh
        let hex_mesh = meshes.add(RegularPolygon::new(elevation_data.hex_radius, 6));

        // Add rendering components to entity
        commands.entity(entity).insert((
            MeshMaterial2d(materials.add(color)),
            Mesh2d(hex_mesh),
            transform.with_rotation(Quat::from_rotation_z(PI / 12.0)),
        ));

        // Create information text as separate entity
        let co = hex_cell.cell.to_coordinate().unwrap();
        let info_text = format!(
            "lat: {:.3}\nlng:{:.3}\nelev: {:.1}",
            co.y, co.x, hex_cell.elevation
        );

        commands.spawn((
            Text2d::new(info_text),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            Transform::from_xyz(transform.translation.x, transform.translation.y, 1.0),
        ));
    }
}

pub fn update_hex_colors(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&HexCell, &MeshMaterial2d<ColorMaterial>)>,
) {
    // Future color update processing when elevation data changes
    for (hex_cell, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let new_color = utils::elevation_to_color(hex_cell.elevation);
            material.color = new_color;
        }
    }
}
