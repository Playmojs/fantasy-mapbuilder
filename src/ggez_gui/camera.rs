use std::collections::HashMap;

use ggez::{mint, winit::window};

use crate::position::Position;

pub struct Cameras {
    pub cameras: HashMap<Camera, Transform>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Camera {
    Map,
    ParentMap,
}

impl Cameras {
    pub fn setup() -> Self {
        let mut cameras: HashMap<Camera, Transform> = HashMap::new();
        cameras.insert(Camera::Map, Transform::new());
        Cameras { cameras }
    }

    pub fn get_transform(&mut self, key: Camera) -> &mut Transform {
        self.cameras.entry(key).or_insert_with(|| Transform::new())
    }

    pub fn get_drawparam<P: Position>(
        &self,
        key: &Camera,
        position: &P,
    ) -> ggez::graphics::DrawParam {
        if let Some(transform) = self.cameras.get(key) {
            ggez::graphics::DrawParam::default()
                .dest(self.transform_position(key, position))
                .scale(mint::Vector2::<f32> {
                    x: transform.scale,
                    y: transform.scale,
                })
        } else {
            ggez::graphics::DrawParam::default()
        }
    }

    pub fn transform_position<P: Position>(&self, key: &Camera, position: &P) -> mint::Point2<f32> {
        // Converts position from camera to screen position

        if let Some(transform) = self.cameras.get(key) {
            mint::Point2::<f32> {
                x: position.x() * transform.scale + transform.dest.x,
                y: position.y() * transform.scale + transform.dest.y,
            }
        } else {
            mint::Point2::<f32> {
                x: position.x(),
                y: position.y(),
            }
        }
    }

    pub fn inv_transform_position<P: Position>(
        // Converts position from screen position to camera position
        &self,
        key: &Camera,
        position: &P,
    ) -> mint::Point2<f32> {
        if let Some(transform) = self.cameras.get(key) {
            mint::Point2::<f32> {
                x: (position.x() - transform.dest.x) / transform.scale,
                y: (position.y() - transform.dest.y) / transform.scale,
            }
        } else {
            mint::Point2::<f32> {
                x: position.x(),
                y: position.y(),
            }
        }
    }

    pub fn is_within<P: Position>
    (
        &self,
        key: &Camera,
        position: &P) -> bool
        {
            self.cameras.get(key).map_or(false, |transform| {
                position.x() > transform.camera_position.x &&
                position.x() < transform.camera_position.x + transform.camera_width_and_height.x &&
                position.y() > transform.camera_position.y &&
                position.y() < transform.camera_position.y + transform.camera_width_and_height.y
            })
        }
}

pub struct Transform {
    pub dest: mint::Point2<f32>,
    pub scale: f32,

    pub dest_min: mint::Point2<f32>,
    pub dest_max: mint::Point2<f32>,

    pub scale_min: f32,
    pub scale_max: f32,

    pub camera_position: mint::Point2<f32>,
    pub camera_width_and_height: mint::Point2<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            dest: mint::Point2::<f32> { x: 0.0, y: 0.0 },
            scale: 1.0,
            dest_min: mint::Point2::<f32> { x: 0.0, y: 0.0 },
            dest_max: mint::Point2::<f32> { x: 0.0, y: 0.0 },
            scale_min: 1.0,
            scale_max: 1.0,
            camera_position: mint::Point2::<f32> {x: 0.0, y: 0.0},
            camera_width_and_height: mint::Point2::<f32> {x: 1.0, y: 1.0},
        }
    }

    pub fn zoom_out(&mut self) -> &mut Self {
        self.scale = self.scale_min;
        self.dest = mint::Point2 { x: 0.0, y: 0.0 };
        self
    }

    pub fn set_limits<P: Position, T: Position>(
        &mut self,
        window_size: &P,
        image_size: &T,
    ) -> &mut Self {
        self.camera_width_and_height = mint::Point2{x: window_size.x(), y: window_size.y()};

        self.scale_min = (window_size.x() / image_size.x()).max(window_size.y() / image_size.y());
        self.scale_max = self.scale_min * 5.0;

        self.dest_max = mint::Point2 { x: 0.0, y: 0.0 };
        self.dest_min = mint::Point2 {
            x: (window_size.x() - self.scale_min * image_size.x()).min(0.0),
            y: (window_size.y() - self.scale_min * image_size.y()).min(0.0),
        };
        self
    }

    pub fn pan<P: Position>(&mut self, movement: &P) {
        self.dest.x = (self.dest.x + movement.x()).clamp(self.dest_min.x, self.dest_max.x);
        self.dest.y = (self.dest.y + movement.y()).clamp(self.dest_min.y, self.dest_max.y);
    }

    pub fn zoom<P: Position>(
        &mut self,
        zoom_increment: &f32,
        zoom_target: &P,
    ) {
        let prev_scale = self.scale;
        self.scale = (self.scale * zoom_increment).clamp(self.scale_min, self.scale_max);

        let scale_ratio = self.scale / prev_scale;

        self.dest_min.x =
            (self.camera_width_and_height.x - (self.camera_width_and_height.x - self.dest_min.x()) * scale_ratio).min(0.0);
        self.dest_min.y =
            (self.camera_width_and_height.y - (self.camera_width_and_height.y - self.dest_min.y()) * scale_ratio).min(0.0);

        self.dest.x = (-(zoom_target.x() - self.dest.x()) * scale_ratio + zoom_target.x())
            .clamp(self.dest_min.x(), self.dest_max.x());
        self.dest.y = (-(zoom_target.y() - self.dest.y()) * scale_ratio + zoom_target.y())
            .clamp(self.dest_min.y(), self.dest_max.y());
    }
}
