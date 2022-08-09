use crate::GameState;
use crate::{CurrentAndNextMap, TiledMap};
use bevy::asset::LoadState;
use bevy::log;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use iyes_loopless::state::NextState;

use super::LoadAssets;
use crate::consts::{MAP_Z, MONSTER_Z};

//加载资源
pub fn load_game_resource(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_and_next_map: Res<CurrentAndNextMap>,
    mut load_assets: ResMut<LoadAssets>,
) {
    let mut raw = Vec::new();

    raw.push(current_and_next_map.current.clone_untyped());
    load_assets.0 = raw;

    log::info!("加载资源");
}

//等待资源加载完成
pub fn wait_resource_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_assets: ResMut<LoadAssets>,
) {
    if load_assets.0.is_empty() {
        return;
    }

    if let LoadState::Loaded =
        asset_server.get_group_load_state(load_assets.0.iter().map(|handle| handle.id))
    {
        load_assets.0 = vec![];

        commands.insert_resource(NextState(GameState::InGame));

        log::info!("资源加载完成");
    }
}

//创建地图
pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_and_next_map: Res<CurrentAndNextMap>,
    tile_maps: Res<Assets<TiledMap>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("wall.png");

    let tilemap_size = TilemapSize { x: 15, y: 13 };

    let tilemap_entity = commands.spawn().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);

    let map = tile_maps.get(&current_and_next_map.current).unwrap();

    // Spawn the elements of the tilemap.
    for (y, lines) in map.iter().rev().enumerate() {
        for (x, index) in lines.iter().enumerate() {
            let tile_pos = TilePos {
                x: x as u32,
                y: y as u32,
            };

            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    texture: TileTexture(*index as u32),
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 15.0, y: 13.0 },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                MAP_Z,
            ),
            ..Default::default()
        });

    log::info!("地图创建完成");
}
