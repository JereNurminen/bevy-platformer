pub mod lever_loader {
    use std::fs;

    use bevy::prelude::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct TldkLayerDefinition {}

    #[derive(Debug, Deserialize)]
    struct TldkEntityDefinition {}

    #[derive(Debug, Deserialize)]
    struct TldkTilesetDefinition {}

    #[derive(Debug, Deserialize)]
    struct TldkDefinitions {
        layers: Vec<TldkLayerDefinition>,
        entities: Vec<TldkEntityDefinition>,
        tilesets: Vec<TldkTilesetDefinition>,
    }

    #[derive(Debug, Deserialize, Clone)]
    struct TldkTileInstance {
        src: (i64, i64),
        px: (i64, i64),
        id: Option<u64>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct TldkEntityInstance {
        #[serde(rename = "__grid")]
        grid: (i64, i64),
        #[serde(rename = "__identifier")]
        identifier: String,
        #[serde(rename = "__tags")]
        tags: Vec<String>,
        #[serde(rename = "__worldX")]
        world_x: i64,
        #[serde(rename = "__worldY")]
        world_y: i64,
        width: Option<u64>,
        height: Option<u64>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "__type")]
    enum TldkLevelLayerInstance {
        IntGrid {
            #[serde(rename = "__cWid")]
            c_wid: u64,
            #[serde(rename = "__cHei")]
            c_hei: u64,
            #[serde(rename = "intGridCsv")]
            int_grid_csv: Vec<u8>,
        },
        #[serde(rename = "AutoLayer")]
        AutoLayer {
            #[serde(rename = "__cWid")]
            c_wid: u64,
            #[serde(rename = "__cHei")]
            c_hei: u64,
            #[serde(rename = "__tilesetDefUid")]
            tileset_def_uid: u64,
            #[serde(rename = "autoLayerTiles")]
            auto_layer_tiles: Vec<TldkTileInstance>,
        },
        #[serde(rename = "Entities")]
        Entities {
            #[serde(rename = "__cWid")]
            c_wid: u64,
            #[serde(rename = "__cHei")]
            c_hei: u64,
            #[serde(rename = "entityInstances")]
            entity_instances: Vec<TldkEntityInstance>,
        },
    }

    struct CollisionGrid {
        grid_width: u64,
        grid_height: u64,
        collider_grid: Vec<u8>,
    }

    struct AutoLayer {
        grid_width: u64,
        grid_height: u64,
        tileset_definion_id: u64,
        tiles: Vec<TldkTileInstance>,
    }

    struct RoomEntities {
        grid_width: u64,
        grid_height: u64,
        entities: Vec<TldkEntityInstance>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TldkLevelDefinition {
        identifier: String,
        uid: u64,
        world_x: i64,
        world_y: i64,
        px_wid: u64,
        px_hei: u64,
    }

    #[derive(Debug, Deserialize)]
    struct WorldRoot {
        defs: TldkDefinitions,
        levels: Vec<TldkLevelDefinition>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct LdtkLevelData {
        uid: u64,
        world_x: i64,
        world_y: i64,
        layer_instances: Vec<TldkLevelLayerInstance>,
    }

    struct RoomData {
        uid: u64,
        world_x: i64,
        world_y: i64,
        collision_tiles: CollisionGrid,
        sprite_tiles: AutoLayer,
        room_entities: RoomEntities,
    }

    const WORLD_DATA_JSON: &str = include_str!("../assets/level-data/tiraic.ldtk");

    fn get_world_data() -> WorldRoot {
        let world_data: WorldRoot = match serde_json::from_str(WORLD_DATA_JSON) {
            Ok(contents) => contents,
            Err(error) => panic!("Parsing level data failed"),
        };
        world_data
    }

    #[derive(Resource)]
    enum CurrentRoom {
        CurrentRoomId(String),
    }

    impl Default for CurrentRoom {
        fn default() -> CurrentRoom {
            CurrentRoom::CurrentRoomId("level_0".to_string())
        }
    }

    #[derive(Bundle)]
    struct TileWithCollisionBundle {
        sprite: SpriteBundle,
    }

    fn load_and_deserialize_room_data(room_id: &String) -> RoomData {
        let room_string =
            fs::read_to_string(format!("./assets/level-data/tiraic/{room_id}.ldtkl")).unwrap();
        print!("{}", room_string);
        let room_data: LdtkLevelData = serde_json::from_str(&room_string).unwrap();
        RoomData {
            uid: room_data.uid,
            world_x: room_data.world_x,
            world_y: room_data.world_y,
            collision_tiles: room_data
                .layer_instances
                .iter()
                .find_map(|layer| match layer {
                    TldkLevelLayerInstance::IntGrid {
                        c_wid: __c_wid,
                        c_hei: __c_hei,
                        int_grid_csv,
                    } => Some(CollisionGrid {
                        grid_width: *__c_wid,
                        grid_height: *__c_hei,
                        collider_grid: int_grid_csv.to_vec(),
                    }),
                    _ => None,
                })
                .unwrap(),
            room_entities: room_data
                .layer_instances
                .iter()
                .find_map(|layer| match layer {
                    TldkLevelLayerInstance::Entities {
                        c_wid: __c_wid,
                        c_hei: __c_hei,
                        entity_instances,
                    } => Some(RoomEntities {
                        grid_width: *__c_wid,
                        grid_height: *__c_hei,
                        entities: entity_instances.to_vec(),
                    }),
                    _ => None,
                })
                .unwrap(),
            sprite_tiles: room_data
                .layer_instances
                .iter()
                .find_map(|layer| match layer {
                    TldkLevelLayerInstance::AutoLayer {
                        c_wid: __c_wid,
                        c_hei: __c_hei,
                        tileset_def_uid: __tileset_def_uid,
                        auto_layer_tiles,
                    } => Some(AutoLayer {
                        grid_width: *__c_wid,
                        grid_height: *__c_hei,
                        tileset_definion_id: *__tileset_def_uid,
                        tiles: auto_layer_tiles.to_vec(),
                    }),
                    _ => None,
                })
                .unwrap(),
        }
    }

    fn generate_entities_for_room(room_id: &String) {}

    fn load_current_room_entities(mut commands: Commands) {
        commands.init_resource::<CurrentRoom>();
        let room_data = load_and_deserialize_room_data(&"level_0".to_string());
        for tile in room_data.sprite_tiles.tiles.into_iter() {}
    }

    pub struct WorldManagementPlugin;

    impl Plugin for WorldManagementPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, load_current_room_entities);
        }
    }
}
