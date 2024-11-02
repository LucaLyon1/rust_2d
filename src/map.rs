use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

const MAP_WIDTH: u32 = 100;
const MAP_HEIGHT: u32 = 100;
const TILE_SIZE: f32 = 32.0;

#[derive(Component, Debug, Clone, Copy)]
enum TileType {
    Water,
    Sand,
    Grass,
    Mountain,
}

#[derive(Component)]
struct Tile {
    tile_type: TileType,
}

#[derive(Resource)]
struct TileTextures {
    water: Handle<Image>,
    sand: Handle<Image>,
    grass: Handle<Image>,
    mountain: Handle<Image>,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load each texture
    let tile_textures = TileTextures {
        water: asset_server.load("water.png"),
        sand: asset_server.load("sand.png"),
        grass: asset_server.load("grass.png"),
        mountain: asset_server.load("mountain.png"),
    };
    generate_map(&mut commands, &tile_textures);
}

fn generate_map(commands: &mut Commands, textures: &TileTextures) {
    let perlin = Perlin::new(1);
    let frequency = 0.1; // Adjust this to scale terrain features

    let offset_x = (MAP_WIDTH as f32 * TILE_SIZE) / -2.0;
    let offset_y = (MAP_HEIGHT as f32 * TILE_SIZE) / -2.0;

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let noise_value = perlin.get([x as f64 * frequency, y as f64 * frequency])
                + 0.5 * perlin.get([x as f64 * frequency * 2.0, y as f64 * frequency * 2.0]);
            let tile_type = if noise_value < -0.2 {
                TileType::Water
            } else if noise_value < 0.0 {
                TileType::Sand
            } else if noise_value < 0.5 {
                TileType::Grass
            } else {
                TileType::Mountain
            };

            let texture = match tile_type {
                TileType::Water => &textures.water,
                TileType::Sand => &textures.sand,
                TileType::Grass => &textures.grass,
                TileType::Mountain => &textures.mountain,
            };

            commands
                .spawn(SpriteBundle {
                    texture: texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            x as f32 * TILE_SIZE + offset_x,
                            y as f32 * TILE_SIZE + offset_y,
                            0.0,
                        ),
                        scale: Vec3::splat(1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Tile { tile_type });
        }
    }
}
