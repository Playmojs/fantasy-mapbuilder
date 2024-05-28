use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectState {
    pub maps: HashMap<MapId, Map>,
}

#[derive(Serialize, Deserialize)]
pub struct Map{   
    pub markers: Vec<Marker>,
    pub map_info: MapInfo,
    pub image: String,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct MapId(pub u64);

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