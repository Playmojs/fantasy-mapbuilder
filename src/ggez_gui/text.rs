use ggez::{graphics::{self, Drawable, PxScale}, mint, Context};

use crate::position::Position;

pub struct TextManager
{
    pub text_handler: graphics::Text,
    pub textbox_size: mint::Vector2::<f32>,
    pub textscale: graphics::PxScale,
}

impl TextManager
{
    pub fn new() -> Self
    {
        TextManager{text_handler: graphics::Text::new(""), textbox_size: mint::Vector2::<f32>{x: 1.0, y: 1.0}, textscale: graphics::PxScale{x: 1.0, y: 1.0}}
    }

    pub fn set_text(&mut self, text: &String) -> &mut Self
    {
        self.text_handler.clear();
        self.text_handler.add(graphics::TextFragment::new(text));
        self
    }

    pub fn set_bounds<P: Position>(&mut self, position: &P) -> &mut Self
    {
        self.text_handler.set_bounds(mint::Vector2::<f32>{x: position.x(), y: position.y()});
        self
    }

    pub fn set_scale<P: Position>(&mut self, position: &P) -> &mut Self
    {
        self.text_handler.set_scale(PxScale{x: position.x(), y: position.y()});
        self
    }

    pub fn set_textbox_size(&mut self, ctx: & Context, y_fill: f32) -> &mut Self
    {
        let rect = self.text_handler.dimensions(ctx).unwrap();
        self.textbox_size = mint::Vector2::<f32> {x: rect.w, y: rect.h + y_fill};
        self
    } 
}