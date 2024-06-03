use std::collections::HashMap;

use ggez::{graphics, mint};

pub struct Cameras {
    pub cameras: HashMap<Camera, Transform>,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Camera {
    Map,
}

impl Cameras {
    pub fn setup() -> Self {
        let mut cameras: HashMap<Camera, Transform> = HashMap::new();
        cameras.insert(Camera::Map, Transform::new());
        Cameras { cameras }
    }

    pub fn get_transform(&mut self, key: &Camera) -> Option<&mut Transform> {
        self.cameras.get_mut(key)
    }

    pub fn get_drawparam(
        &self,
        key: &Camera,
        position: &mint::Point2<f32>,
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

    pub fn transform_position(
        &self,
        key: &Camera,
        position: &mint::Point2<f32>,
    ) -> mint::Point2<f32> {
        if let Some(transform) = self.cameras.get(key) {
            mint::Point2::<f32> {
                x: position.x * transform.scale + transform.dest.x,
                y: position.y * transform.scale + transform.dest.y,
            }
        } else {
            *position
        }
    }

    pub fn inv_transform_position(
        &self,
        key: &Camera,
        position: &mint::Point2<f32>,
    ) -> mint::Point2<f32> {
        if let Some(transform) = self.cameras.get(key) {
            mint::Point2::<f32> {
                x: (position.x - transform.dest.x) / transform.scale,
                y: (position.y - transform.dest.y) / transform.scale,
            }
        } else {
            *position
        }
    }
}

pub struct Transform {
    pub dest: mint::Point2<f32>,
    pub scale: f32,

    pub dest_min: mint::Point2<f32>,
    pub dest_max: mint::Point2<f32>,

    pub scale_min: f32,
    pub scale_max: f32,
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
        }
    }

    pub fn zoom_out(&mut self) -> &mut Self {
        self.scale = self.scale_min;
        self.dest = mint::Point2 { x: 0.0, y: 0.0 };
        self
    }

    pub fn set_limits(
        &mut self,
        window_size: mint::Vector2<f32>,
        image_size: mint::Vector2<f32>,
    ) -> &mut Self {
        self.scale_min = (window_size.x / image_size.x).max(window_size.y / image_size.y);
        self.scale_max = self.scale_max * 5.0;

        self.dest_max = mint::Point2 { x: 0.0, y: 0.0 };
        self.dest_min = mint::Point2 {
            x: (window_size.x - self.scale_min * image_size.x).min(0.0),
            y: (window_size.y - self.scale_min * image_size.y).min(0.0),
        };
        self
    }

    pub fn pan(&mut self, movement: mint::Point2<f32>) {
        self.dest.x = (self.dest.x + movement.x).clamp(self.dest_min.x, self.dest_max.x);
        self.dest.y = (self.dest.y + movement.y).clamp(self.dest_min.y, self.dest_max.y);
    }

    pub fn zoom(
        &mut self,
        zoom_increment: f32,
        window_size: mint::Vector2<f32>,
        image_size: mint::Vector2<f32>,
        zoom_target: mint::Point2<f32>,
    ) {
        let prev_scale = self.scale;
        self.scale = (self.scale * zoom_increment).clamp(self.scale_min, self.scale_max);

        self.dest_min = mint::Point2 {
            x: (window_size.x - image_size.x * self.scale).min(0.0),
            y: (window_size.y - image_size.y * self.scale).min(0.0),
        };

        let scale_ratio = self.scale / prev_scale;
        self.dest.x = (-(zoom_target.x - self.dest.x) * scale_ratio + zoom_target.x)
            .clamp(self.dest_min.x, self.dest_max.x);
        self.dest.y = (-(zoom_target.y - self.dest.y) * scale_ratio + zoom_target.y)
            .clamp(self.dest_min.y, self.dest_max.y);
    }
}
