use std::collections::HashMap;

use common::project_state::{Map, MapId, MapInfo, Marker, ProjectState};
use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};

struct MainState {
    image: graphics::Image,
    project_state: ProjectState,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let image = graphics::Image::from_path(ctx, "/map.jpg")?;

        let mut maps = HashMap::<MapId, Map>::new();
        let map = Map {
            markers: Vec::new(),
            map_info: MapInfo {
                content: "".to_string(),
            },
            image: "assets/map.jpg".to_string(),
        };
        maps.insert(MapId { 0: 0 }, map);

        Ok(MainState {
            image,
            project_state: ProjectState { maps },
        })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        canvas.draw(&self.image, graphics::DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("lpenlpen", "ggez").resources_dir_name("../../assets");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
