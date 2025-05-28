use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    q: i32, // rows
    r: i32, // columns
}