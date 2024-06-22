// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{path::Path, sync::Mutex};

use common::{ids::MapId, project_state::{Marker, ProjectState}};
use comrak::{markdown_to_html, ComrakOptions};
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
struct FrontEndMap
{
  pub markers: Vec<Marker>,
  pub text: String,
  pub image: String,
  pub parent_image: Option<String>,
  pub parent_id: Option<MapId>,
}


#[tauri::command]
fn get_text() -> String {
  let mut project_state = ProjectState::load(Path::new("./Projects/test"));
  project_state.current_map = MapId(1);
  markdown_to_html(project_state.current_map().map_info.content.as_str(), &ComrakOptions::default())
}

#[tauri::command]
fn set_map(project_state: State<Mutex<ProjectState>>, id: &str) -> FrontEndMap
{
  let mut project_state = project_state.lock().unwrap();
  let id: u64 = id.parse::<u64>().ok().unwrap();

  let current_id = project_state.current_map.clone();
  project_state.map_history_stack.push(current_id);
  project_state.current_map = MapId(id);
  let map = project_state.current_map();
  let parent_string = map.parent_id.clone().map(|id| project_state.maps.get(&id).unwrap().image.clone());
  FrontEndMap{markers: map.markers.values().cloned().collect::<Vec<Marker>>(), text: map.map_info.content.clone(), image: map.image.clone(), parent_image: parent_string, parent_id: map.parent_id.clone()}
}

#[tauri::command]
fn pop_map(project_state: State<Mutex<ProjectState>>) -> Option<FrontEndMap>
{
  let mut project_state = project_state.lock().unwrap();

  project_state.map_history_stack.pop();

  let current_map = project_state.map_history_stack.last()?.clone();
  project_state.current_map = current_map;
  let map = project_state.current_map();
  let parent_string = map.parent_id.clone().map(|id| project_state.maps.get(&id).unwrap().image.clone());
  Some(FrontEndMap{markers: map.markers.values().cloned().collect::<Vec<Marker>>(), text: map.map_info.content.clone(), image: map.image.clone(), parent_image: parent_string, parent_id: map.parent_id.clone()})
}

#[tauri::command]
fn update_map_content(project_state: State<Mutex<ProjectState>>, content: &str){
  let mut project_state = project_state.lock().unwrap();
  let current_map = project_state.current_map.clone();
  project_state.maps.get_mut(&current_map).unwrap().map_info.content = content.to_string();
  project_state.save(Path::new("./Projects/test"))
}

fn main() {
  let mut project_state = ProjectState::load(Path::new("./Projects/test"));

  tauri::Builder::default()
    .manage(Mutex::new(project_state))
    .invoke_handler(tauri::generate_handler![get_text, set_map, pop_map, update_map_content])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
