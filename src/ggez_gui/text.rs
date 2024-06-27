use ggez::{graphics::{self, Drawable, PxScale, Text}, mint, Context};

use crate::{camera::Camera, position::Position};

pub struct TextManager
{
    pub text_handler: graphics::Text,
    pub text_size: (f32, f32),
    pub textbox_dims: (f32, f32),
    pub textscale: graphics::PxScale,
    pub text_editor: TextEditor,
}

impl TextManager
{
    pub fn new() -> Self
    {
        TextManager{text_handler: graphics::Text::new(""), text_size: (1.0, 1.0), textscale: graphics::PxScale{x: 1.0, y: 1.0}, text_editor: TextEditor{text: "".to_string(), indicator: 0}, textbox_dims: (0.3, 0.7)}
    }

    pub fn set_text(&mut self, text: &String) -> &mut Self
    {
        self.text_editor.text = text.clone();
        self.text_handler.clear();
        self.text_handler.add(graphics::TextFragment::new(text));
        self
    }

    pub fn set_bounds(&mut self, ctx: &Context) -> &mut Self
    {
        self.text_handler.set_bounds(mint::Vector2::<f32>{x: self.textbox_dims.0 * ctx.gfx.drawable_size().0 * 0.9, y: self.textbox_dims.0*10000000.0 * ctx.gfx.drawable_size().1});
        self
    }

    pub fn set_scale<P: Position>(&mut self, position: &P) -> &mut Self
    {
        self.text_handler.set_scale(PxScale{x: position.x(), y: position.y()});
        self
    }

    pub fn compute_text_size(&mut self, ctx: & Context, y_fill: f32) -> &mut Self
    {
        let rect = self.text_handler.dimensions(ctx).unwrap();
        self.text_size = (rect.w, rect.h + y_fill);
        self
    }
    
    pub fn arrange_camera(&self, ctx: & Context, camera: &mut Camera) -> & Self
    {
        let min_x = 0.26 * ctx.gfx.drawable_size().0;
        let min_y = 0.7 * ctx.gfx.drawable_size().1;

        let mut indicator_y = 0.0;
        if self.text_editor.text.len() > 0{
            let ind = self.text_editor.indicator.min(self.text_handler.glyph_positions(ctx).ok().unwrap().len() - 1);
            // println!("Glyph len: {}", self.text_handler.glyph_positions(ctx).ok().unwrap().len());
            // println!("Str len: {}", self.text_editor.text.len());
            indicator_y = (self.text_handler.glyph_positions(ctx).ok().unwrap().get(ind).unwrap().y) * -1.1;
        }
        camera.scale_to_fit_horizontal(
            &(ctx.gfx.drawable_size().0 * &self.textbox_dims.0, ctx.gfx.drawable_size().1 * &self.textbox_dims.1),
            &(self.text_size.0.max(min_x), self.text_size.1.max(min_y)),
            &(ctx.gfx.drawable_size().0 * 0.7, ctx.gfx.drawable_size().1 * 0.3),
            indicator_y
        );
        self
    }

    pub fn set_text_from_editor(&mut self) -> &mut Self
    {
        self.text_handler.clear();
        self.text_handler.add(graphics::TextFragment::new(self.text_editor.text.clone()));
        self
    }
}

pub struct TextEditor
{
    pub text: String,
    pub indicator: usize,
}
