use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::OsStr, path::Path};

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectState {
    pub maps: HashMap<MapId, Map>,
    pub current_map: MapId,
    pub map_history_stack: Vec<MapId>,
}

impl ProjectState {
    pub fn current_map(&self) -> &Map {
        return self.maps.get(&self.current_map).unwrap();
    }
    pub fn load(project_dir: &Path) -> Self {
        let maps: HashMap<_, _> = project_dir
            .into_iter()
            .filter_map(|file_path| {
                let FileType::Map(id) = get_filetype(file_path) else {
                    return None;
                };
                let map = Map::load(file_path)?;
                Some((id, map))
            })
            .collect();
        Self {
            maps,
            ..Default::default()
        }
    }
}

pub enum FileType {
    Map(MapId),
    Marker(MarkerId),
    None,
}

pub fn get_filetype(file_path: &OsStr) -> FileType {
    let cow = file_path.to_string_lossy();
    let mut file_name_iter = cow.split("-");
    match (
        file_name_iter.next(),
        file_name_iter
            .next()
            .and_then(|id_string| id_string.parse::<u64>().ok()),
    ) {
        (Some("map"), Some(id)) => FileType::Map(MapId::new(id)),
        (Some("marker"), Some(id)) => FileType::Marker(MarkerId::new(id)),
        _ => FileType::None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub markers: HashMap<MarkerId, Marker>,
    pub map_info: MapInfo,
    pub image: String,
}

impl Map {
    pub fn load(filepath: &OsStr) -> Option<Map> {
        None
    }
}

macro_rules! define_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, Default)]
        pub struct $name(pub u64);

        impl $name {
            pub fn new(raw: u64) -> Self {
                Self(raw)
            }
        }
    };
}

define_id!(MapId);
define_id!(MarkerId);

#[derive(Serialize, Deserialize)]
pub struct Marker {
    pub map_id: MapId,
    pub position: Position,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MapInfo {
    pub content: String,
}
