use ggez::glam::Vec2;
use ggez::mint::Point2;
use ggez::mint::Vector2;

pub trait Position {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl Position for (f32, f32) {
    fn x(&self) -> f32 {
        self.0
    }

    fn y(&self) -> f32 {
        self.1
    }
}

impl Position for Point2<f32> {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Position for Vector2<f32> {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Position for Vec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}
