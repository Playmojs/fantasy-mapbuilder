use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectState {
    pub maps: HashMap<MapId, Map>,
    pub current_map: MapId,
    pub previous_map: MapId,
}

impl ProjectState {
    pub fn current_map(&self) -> &Map
    {
        return self.maps.get(&self.current_map).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct Map{   
    pub markers: Vec<Marker>,
    pub map_info: MapInfo,
    pub image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct MapId(pub u64);

impl MapId{
    pub fn new(raw: u64)->Self{
        Self{
            0: raw,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Marker{
    pub map_id: MapId,
    pub position: Position,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct MapInfo{
    pub content: String,
}