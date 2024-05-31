use std::{
    collections::{hash_map::Entry, HashMap},
    marker,
    path::Path,
};
mod config;
use common::project_state::ProjectState;
use config::MARKER_SIZE;
use ggez::{
    context::Has,
    event::{self, Button},
    glam::Vec2,
    graphics::{self, DrawParam},
    mint::{self, Point2, Vector2},
    Context, GameError, GameResult,
};
use serde_json::{from_slice, map::OccupiedEntry};
use slint::Image;

struct MainState {
    images: HashMap<String, graphics::Image>,
    project_state: ProjectState,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(MainState {
            images: HashMap::new(),
            project_state: ProjectState::load(Path::new("./Projects/test")),
        })
    }
    fn get_image(&mut self, ctx: &Context, path: &String) -> &graphics::Image {
        self.images
            .entry(path.clone())
            .or_insert_with(|| graphics::Image::from_path(ctx, path).unwrap())
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let project_dir: &Path = Path::new("./Projects/test");
        let position: Vec2 = ctx.mouse.position().into();
        let map = self.project_state.current_map();

        if ctx.mouse.button_just_released(event::MouseButton::Left) {
            if let Some(map_id) = map
                .markers
                .iter()
                .find(|(_, marker)| {
                    (Vec2::new(marker.position.x, marker.position.y) - position).length()
                        < MARKER_SIZE
                })
                .map(|(_, hovered_marker)| hovered_marker.map_id.clone())
            {
                self.project_state
                    .map_history_stack
                    .push(self.project_state.current_map.clone());
                self.project_state.current_map = map_id.clone();
            }
        }
        if ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::Back)
        {
            if let Some(previous) = self.project_state.map_history_stack.pop() {
                self.project_state.current_map = previous;
            }
        }
        if ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::S)
        {
            self.project_state.save(project_dir)
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let image = self.get_image(ctx, &self.project_state.current_map().image.clone());
        canvas.draw(image, graphics::DrawParam::default());

        for (_, marker) in &self
            .project_state
            .maps
            .get(&self.project_state.current_map)
            .unwrap()
            .markers
        {
            let position = mint::Point2::<f32> {
                x: marker.position.x,
                y: marker.position.y,
            };
            canvas.draw(
                &graphics::Image::from_path(ctx, &marker.image)?,
                DrawParam::default()
                    .dest(position)
                    .offset(Point2 { x: 0.5, y: 1.0 }),
            );
        }

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
