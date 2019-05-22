use graphics::{Context, Image, Transformed};
use opengl_graphics::{GlGraphics, Texture};

pub enum Pickup {
    Cherry(u32, u32),
    Apple(u32, u32),
}

impl Pickup {
    pub fn new_cherry(x: u32, y: u32) -> Self {
        Pickup::Cherry(x, y)
    }

    pub fn new_apple(x: u32, y: u32) -> Self {
        Pickup::Apple(x, y)
    }

    pub fn render(&self, gl: &mut GlGraphics, context: &Context) {}
}
