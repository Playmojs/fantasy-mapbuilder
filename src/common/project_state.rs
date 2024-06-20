use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};

use crate::{
    file_type::{get_file_type, get_filename, FileType},
    ids::*,
};

#[derive(Serialize, Deserialize, Default)]
pub struct ProjectState {
    pub current_map: MapId,

    #[serde(skip)]
    pub maps: HashMap<MapId, Map>,

    #[serde(skip)]
    pub map_history_stack: Vec<MapId>,
}

impl ProjectState {
    pub fn current_map(&self) -> &Map {
        return self.maps.get(&self.current_map).unwrap();
    }
    pub fn load(project_dir: &Path) -> Self {
        let maps: HashMap<_, _> = std::fs::read_dir(project_dir)
            .unwrap()
            .filter_map(|file_path| {
                let file_path = file_path.unwrap().path();
                let Some(FileType::Map(id)) = get_file_type(&file_path) else {
                    return None;
                };
                let map = Map::load(project_dir, &file_path).unwrap();
                Some((id, map))
            })
            .collect();
        Self {
            maps,
            ..Default::default()
        }
    }

    pub fn save(&self, project_dir: &Path) {
        for (map_id, map) in &self.maps {
            map.save(
                project_dir,
                project_dir
                    .join(&get_filename(FileType::Map(map_id.clone())))
                    .as_path(),
            );
        }
    }
}

pub struct Map {
    pub markers: HashMap<MarkerId, Marker>,
    pub map_info: MapInfo,
    pub image: String,
    pub parent_id: Option<MapId>,
}

impl Map {
    pub fn load(project_dir: &Path, file_path: &Path) -> Option<Map> {
        let map_on_file = MapOnFile::load(file_path).unwrap();
        Some(Self {
            markers: map_on_file
                .marker_ids
                .into_iter()
                .filter_map(|marker_id| {
                    Some((
                        marker_id.clone(),
                        Marker::load(
                            project_dir
                                .join(&get_filename(FileType::Marker(marker_id)))
                                .as_path(),
                        )
                        .unwrap(),
                    ))
                })
                .collect(),
            map_info: map_on_file.map_info,
            image: map_on_file.image,
            parent_id: map_on_file.parent_id,
        })
    }

    pub fn save(&self, project_dir: &Path, file_path: &Path) {
        for (marker_id, marker) in &self.markers {
            marker.save(
                project_dir
                    .join(&get_filename(FileType::Marker(marker_id.clone())))
                    .as_path(),
            );
        }

        _ = MapOnFile {
            marker_ids: self.markers.keys().cloned().collect(),
            map_info: self.map_info.clone(),
            image: self.image.clone(),
            parent_id: self.parent_id.clone(),
        }
        .save(file_path)
    }
}

impl MapOnFile {
    pub fn load(file_path: &Path) -> Option<MapOnFile> {
        let file = fs::File::open(file_path).ok().unwrap();
        let buf_reader = std::io::BufReader::new(file);
        Some(serde_json::from_reader(buf_reader).unwrap())
    }

    pub fn save(&self, file_path: &Path) {
        let file = fs::File::create(file_path).unwrap();
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer(buf_writer, &self).unwrap();
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapOnFile {
    pub marker_ids: Vec<MarkerId>,
    pub map_info: MapInfo,
    pub image: String,
    pub parent_id: Option<MapId>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Marker {
    pub map_id: MapId,
    pub position: Position,
    pub image: String,
}

impl Marker {
    pub fn load(file_path: &Path) -> Option<Self> {
        let file = fs::File::open(file_path).ok().unwrap();
        let buf_reader = std::io::BufReader::new(file);
        Some(serde_json::from_reader(buf_reader).unwrap())
    }
    pub fn save(&self, file_path: &Path) {
        let file = fs::File::create(file_path).unwrap();
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer(buf_writer, &self).unwrap();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapInfo {
    pub content: String,
}
