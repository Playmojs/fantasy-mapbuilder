use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::OsStr, fs, io::BufReader, path::Path};

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
        let maps: HashMap<_, _> = project_dir
            .into_iter()
            .filter_map(|file_path| {
                let FileType::Map(id) = get_filetype(file_path) else {
                    return None;
                };
                let map = Map::load(project_dir, Path::new(file_path))?;
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
                    .with_file_name(&get_filename(FileType::Map(map_id.clone())))
                    .as_path(),
            );
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

pub fn get_filename(filetype: FileType) -> String {
    match filetype {
        FileType::Map(map_id) => format!("map-{}.json", map_id.0),
        FileType::Marker(marker_id) => format!("marker-{}.json", marker_id.0),
        FileType::None => panic!(),
    }
}

pub struct Map {
    pub markers: HashMap<MarkerId, Marker>,
    pub map_info: MapInfo,
    pub image: String,
}

impl Map {
    pub fn load(project_dir: &Path, file_path: &Path) -> Option<Map> {
        let map_on_file = MapOnFile::load(file_path)?;
        Some(Self {
            markers: map_on_file
                .marker_ids
                .into_iter()
                .filter_map(|marker_id| {
                    Some((
                        marker_id.clone(),
                        Marker::load(
                            project_dir
                                .with_file_name(&get_filename(FileType::Marker(marker_id)))
                                .as_path(),
                        )?,
                    ))
                })
                .collect(),
            map_info: map_on_file.map_info,
            image: map_on_file.image,
        })
    }

    pub fn save(&self, project_dir: &Path, file_path: &Path) {
        for (marker_id, marker) in &self.markers {
            _ = marker.save(
                project_dir
                    .with_file_name(&get_filename(FileType::Marker(marker_id.clone())))
                    .as_path(),
            );
        }

        _ = MapOnFile {
            marker_ids: self.markers.keys().cloned().collect(),
            map_info: self.map_info.clone(),
            image: self.image.clone(),
        }
        .save(file_path)
    }
}

impl MapOnFile {
    pub fn load(file_path: &Path) -> Option<MapOnFile> {
        let file = fs::File::open(file_path).ok()?;
        let buf_reader = std::io::BufReader::new(file);
        serde_json::from_reader(buf_reader).ok()
    }

    pub fn save(&self, file_path: &Path) -> Result<(), std::io::Error> {
        let file = fs::File::create(file_path)?;
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer(buf_writer, &self);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MapOnFile {
    pub marker_ids: Vec<MarkerId>,
    pub map_info: MapInfo,
    pub image: String,
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

impl Marker {
    pub fn load(file_path: &Path) -> Option<Self> {
        let file = fs::File::open(file_path).ok()?;
        let buf_reader = std::io::BufReader::new(file);
        serde_json::from_reader(buf_reader).ok()
    }
    pub fn save(&self, file_path: &Path) -> Result<(), std::io::Error> {
        let file = fs::File::create(file_path)?;
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer(buf_writer, &self);
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MapInfo {
    pub content: String,
}
