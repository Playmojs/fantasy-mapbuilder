use std::{collections::HashMap, path::Path};
mod config;
use camera::{CameraId, CameraManager};
use common::{ids::MapId, project_state::ProjectState};
use config::{MARKER_SIZE, PARENT_MAP_X_RATIO};
use ggez::{
    event, glam::Vec2, graphics::{self, DrawParam, FillOptions}, input::keyboard::{KeyCode, KeyInput, KeyMods}, mint::{self, Point2, Vector2}, Context, GameError, GameResult
};
use rand::Fill;
use serde_json::{from_slice, map::OccupiedEntry};
use slint::Image;
use text::TextManager;

mod camera;
mod position;
mod text;

fn get_image_size(image: &graphics::Image) -> mint::Vector2<f32> {
    mint::Vector2::<f32> {
        x: image.width() as f32,
        y: image.height() as f32,
    }
}

struct MainState {
    images: HashMap<String, graphics::Image>,
    project_state: ProjectState,
    camera_manager: CameraManager,
    text_manager: TextManager,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(MainState {
            images: HashMap::<String, graphics::Image>::new(),
            project_state: ProjectState::load(Path::new("./Projects/test")),
            camera_manager: CameraManager::new(),
            text_manager: TextManager::new(),
        })
    }
    fn get_image(&mut self, ctx: &Context, path: &String) -> &graphics::Image {
        self.images
            .entry(path.clone())
            .or_insert_with(|| graphics::Image::from_path(ctx, path).unwrap())
    }

    fn set_current_map(&mut self, ctx: &Context, map_id: MapId) {
        self.project_state
            .map_history_stack
            .push(self.project_state.current_map.clone());
        self.project_state.current_map = map_id;
        let image_size =
            get_image_size(self.get_image(ctx, &self.project_state.current_map().image.clone()));

        self.camera_manager
            .get_camera(CameraId::Map)
            .set_limits(&ctx.gfx.drawable_size(), &image_size, &(0.0, 0.0))
            .zoom_out();

        if let Some(parent_id) = self.project_state.current_map().parent_id.clone() {
            let parent_size = get_image_size(
                self.get_image(
                    ctx,
                    &self
                        .project_state
                        .maps
                        .get(&parent_id)
                        .unwrap()
                        .image
                        .clone(),
                ),
            );
            self.camera_manager
                .get_camera(CameraId::ParentMap)
                .set_limits(
                    &(
                        ctx.gfx.drawable_size().0 * PARENT_MAP_X_RATIO,
                        ctx.gfx.drawable_size().0 * PARENT_MAP_X_RATIO * parent_size.y / parent_size.x,
                    ),
                    &parent_size,
                    &(0.0, 0.0),
                )
                .zoom_out();
        }
       
        self.text_manager
            .set_text(&self.project_state.current_map().map_info.content)
            .set_bounds(ctx)
            .set_scale(&(35.0, 35.0))
            .compute_text_size(ctx, 50.0)
            .arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let project_dir: &Path = Path::new("./Projects/test");
        let mouse_pos = ctx.mouse.position();
        let position: Vec2 = self
            .camera_manager
            .inv_transform_position(&CameraId::Map, &mouse_pos)
            .into();
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
                self.set_current_map(ctx, map_id)
            }

            // This is currently buggy - if a marker is beneath the parent of either the current map or the target map, this clause will fail because of the change made above.
            if let Some(parent_id) = self.project_state.current_map().parent_id.clone() {
                if self
                    .camera_manager
                    .is_within(&CameraId::ParentMap, &mouse_pos)
                {
                    self.set_current_map(ctx, parent_id);
                }
            }
        }
        if ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::Backslash)
        {
            if let Some(previous) = self.project_state.map_history_stack.pop() {
                self.set_current_map(ctx, previous)
            }
        }
        if ctx
            .keyboard
            .is_key_just_pressed(ggez::input::keyboard::KeyCode::S)
        {
            self.project_state.save(project_dir)
        }

        if ctx.mouse.button_pressed(event::MouseButton::Left) {
            self.camera_manager
                .get_camera(CameraId::Map)
                .pan(&ctx.mouse.delta());
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from_rgb(35, 35, 35));


        // Draw main map
        let map_param = self
            .camera_manager
            .get_draw_param(&CameraId::Map, &mint::Point2::<f32> { x: 0.0, y: 0.0 });
        let image = self.get_image(ctx, &self.project_state.current_map().image.clone());

        canvas.draw(image, map_param);


        // Draw markers
        for marker in self.project_state.current_map().markers.values() {
            let position = mint::Point2::<f32> {
                x: marker.position.x,
                y: marker.position.y,
            };
            canvas.draw(
                &graphics::Image::from_path(ctx, &marker.image)?,
                self.camera_manager
                    .get_draw_param(&CameraId::Map, &position)
                    .offset(Point2 { x: 0.5, y: 1.0 }),
            );
        }

        // Draw parent map
        if let Some(parent_id) = self.project_state.current_map().parent_id.clone() {
            let draw_param = self
                .camera_manager
                .get_draw_param(&CameraId::ParentMap, &(0.0, 0.0));
            canvas.draw(
                self.get_image(
                    ctx,
                    &self
                        .project_state
                        .maps
                        .get(&parent_id)
                        .unwrap()
                        .image
                        .clone(),
                ),
                draw_param,
            );
        }

        // Draw textbox - background
        
        canvas.draw(
            &graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Fill(FillOptions::DEFAULT),
                graphics::Rect::new(0.0, 0.0, self.text_manager.text_size.0 + 20.0, self.text_manager.text_size.1 + 10.0),
                graphics::Color::from_rgb(35, 35, 35),
            )
            .ok()
            .unwrap(),
            self.camera_manager
                .get_draw_param(&CameraId::TextWindow, &(-10.0, -10.0)),
        );

        canvas.draw(
            &graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::Fill(FillOptions::DEFAULT),
                graphics::Rect::new(0.0, 0.0, self.text_manager.text_size.0 - 5.0, self.text_manager.text_size.1 - 30.0),
                graphics::Color::from_rgb(150, 150, 150),
            )
            .ok()
            .unwrap(),
            self.camera_manager
                .get_draw_param(&CameraId::TextWindow, &(0.0, 0.0)),
        );

        // Draw text
        canvas.draw(
         &self.text_manager.text_handler,
            self.camera_manager
                .get_draw_param(&CameraId::TextWindow, &(0.0, 0.0)),
        );

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        if self
            .camera_manager
            .is_within(&CameraId::TextWindow, &ctx.mouse.position())
        {
            self.camera_manager
                .get_camera(CameraId::TextWindow)
                .pan(&(0.0, 75.0 * y));
        } else {
            self.camera_manager.get_camera(CameraId::Map).zoom(
                &(1.0 + y / 10.0),
                &ctx.mouse.position(),
                true,
            );
        }
        Ok(())
    }

    fn resize_event(
        &mut self,
        ctx: &mut Context,
        width: f32,
        height: f32,
    ) -> Result<(), ggez::GameError> {
        let image_size =
            get_image_size(self.get_image(ctx, &self.project_state.current_map().image.clone()));
        self.camera_manager
            .get_camera(CameraId::Map)
            .set_limits(&(width, height), &image_size, &(0.0, 0.0))
            .zoom_out();
        if let Some(parent_id) = self.project_state.current_map().parent_id.clone() {
            let parent_size = get_image_size(
                self.get_image(
                    ctx,
                    &self
                        .project_state
                        .maps
                        .get(&parent_id)
                        .unwrap()
                        .image
                        .clone(),
                ),
            );
            self.camera_manager
                .get_camera(CameraId::ParentMap)
                .set_limits(
                    &(
                        ctx.gfx.drawable_size().0 * PARENT_MAP_X_RATIO,
                        ctx.gfx.drawable_size().0 * PARENT_MAP_X_RATIO * parent_size.y / parent_size.x,
                    ),
                    &parent_size,
                    &(0.0, 0.0),
                )
                .zoom_out();
            self.text_manager
                .set_text(&self.project_state.current_map().map_info.content)
                .set_bounds(ctx)
                .compute_text_size(ctx, 100.0)
                .arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
        };
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyInput, _repeat: bool) -> Result<(), GameError> {
        let keycode = keycode.keycode.unwrap();
        match keycode {
            KeyCode::Back => {
                if self.text_manager.text_editor.indicator > 0 {
                    self.text_manager.text_editor.text.remove(self.text_manager.text_editor.indicator - 1);
                    self.text_manager.text_editor.indicator -= 1;
                    self.text_manager.set_text_from_editor().compute_text_size(ctx, 50.0).arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
                }
            }
            KeyCode::Left => {
                if self.text_manager.text_editor.indicator > 0 {
                    self.text_manager.text_editor.indicator -= 1;
                    self.text_manager.arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
                }
            }
            KeyCode::Right => {
                if self.text_manager.text_editor.indicator < self.text_manager.text_editor.text.len() {
                    self.text_manager.text_editor.indicator += 1;
                    self.text_manager.arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
                }
            }
            KeyCode::End => {
                self.text_manager.text_editor.indicator = self.text_manager.text_editor.text.len();
                self.text_manager.arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
            }
            KeyCode::Home => {
                self.text_manager.text_editor.indicator = 0;
                self.text_manager.arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
            }
            _ => {}
        }
        Ok(())
    }

    fn text_input_event(&mut self, ctx: &mut Context, character: char) -> Result<(), GameError> {
        if character != 8.into()
        {
            self.text_manager.text_editor.text.insert(self.text_manager.text_editor.indicator, character);
            self.text_manager.text_editor.indicator += 1;

            self.text_manager.set_text_from_editor().compute_text_size(ctx, 50.0).arrange_camera(ctx, self.camera_manager.get_camera(CameraId::TextWindow));
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("lpenlpen", "ggez").resources_dir_name("../../assets");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
