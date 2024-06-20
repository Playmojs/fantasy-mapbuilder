use std::{collections::HashMap, path::Path};
mod config;
use camera::{CameraId, CameraManager};
use common::{ids::MapId, project_state::ProjectState};
use config::{MARKER_SIZE, PARENT_MAP_X_RATIO};
use ggez::{
    event,
    glam::Vec2,
    graphics,
    mint::{self, Point2},
    Context, GameResult,
};

mod camera;
mod position;

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
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(MainState {
            images: HashMap::<String, graphics::Image>::new(),
            project_state: ProjectState::load(Path::new("./Projects/test")),
            camera_manager: CameraManager::new(),
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
            .set_limits(&ctx.gfx.drawable_size(), &image_size)
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
                        ctx.gfx.size().0 * PARENT_MAP_X_RATIO,
                        ctx.gfx.size().0 * PARENT_MAP_X_RATIO * parent_size.y / parent_size.x,
                    ),
                    &parent_size,
                )
                .zoom_out();
        }
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

        if ctx.mouse.button_pressed(event::MouseButton::Left) {
            self.camera_manager
                .get_camera(CameraId::Map)
                .pan(&ctx.mouse.delta())
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        let map_param = self
            .camera_manager
            .get_draw_param(&CameraId::Map, &mint::Point2::<f32> { x: 0.0, y: 0.0 });
        let image = self.get_image(ctx, &self.project_state.current_map().image.clone());

        canvas.draw(image, map_param);

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

        canvas.finish(ctx)?;

        Ok(())
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, _x: f32, y: f32) -> GameResult {
        self.camera_manager
            .get_camera(CameraId::Map)
            .zoom(&(1.0 + y / 10.0), &ctx.mouse.position());
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
            .set_limits(&(width, height), &image_size)
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
                        ctx.gfx.size().0 * PARENT_MAP_X_RATIO,
                        ctx.gfx.size().0 * PARENT_MAP_X_RATIO * parent_size.y / parent_size.x,
                    ),
                    &parent_size,
                )
                .zoom_out();
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
