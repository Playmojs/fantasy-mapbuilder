use std::path::Path;

use itertools::Itertools;

use crate::ids::*;

pub enum FileType {
    Map(MapId),
    Marker(MarkerId),
}

pub fn get_file_type(file_path: &Path) -> Option<FileType> {
    let cow = file_path.file_stem()?.to_string_lossy();
    let (prefix, id) = cow.split("-").next_tuple()?;
    match (prefix, id.parse::<u64>().ok()) {
        ("map", Some(id)) => Some(FileType::Map(MapId::new(id))),
        ("marker", Some(id)) => Some(FileType::Marker(MarkerId::new(id))),
        _ => None,
    }
}

pub fn get_filename(filetype: FileType) -> String {
    match filetype {
        FileType::Map(map_id) => format!("map-{}.json", map_id.0),
        FileType::Marker(marker_id) => format!("marker-{}.json", marker_id.0),
    }
}
