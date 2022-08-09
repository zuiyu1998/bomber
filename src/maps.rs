use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

use bevy_ecs_tilemap::prelude::*;

pub struct CurrentAndNextMap {
    pub level: usize,
    pub current: Handle<TiledMap>,
    pub next: Option<Handle<TiledMap>>,
    pub prev: Option<Handle<TiledMap>>,
}

impl CurrentAndNextMap {
    pub fn get_path(&self) -> String {
        format!("{:03}", self.level)
    }
}

impl FromWorld for CurrentAndNextMap {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let current = asset_server.load("maps/001.map");

        CurrentAndNextMap {
            level: 1,
            current,
            next: None,
            prev: None,
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .add_asset::<TiledMap>()
            .add_asset_loader(MapLoader)
            .init_resource::<CurrentAndNextMap>();
    }
}

pub struct MapLoader;

#[derive(TypeUuid, Deref, Debug)]
#[uuid = "e51081d0-6168-4881-a1c6-4249b2000d7f"]
pub struct TiledMap(Vec<Vec<usize>>);

impl AssetLoader for MapLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let raw = String::from_utf8_lossy(bytes);

            let mut raw_tile_map = Vec::new();

            for line in raw.lines() {
                let mut tmp = vec![];

                for index in line.chars() {
                    tmp.push(index as usize - 48);
                }

                raw_tile_map.push(tmp);
            }

            let loaded_asset = LoadedAsset::new(TiledMap(raw_tile_map));
            load_context.set_default_asset(loaded_asset);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["map"];
        EXTENSIONS
    }
}
