use std::ops::DerefMut;
use graphics::{Context, Image, Transformed, Viewport};
use opengl_graphics::{GlGraphics, OpenGL, Texture};

pub trait GenericContext {
    type Context: GenericContext;

    fn inner_mut(&mut self) -> &mut Self::Context;

    fn size(&self) -> (u32, u32);

    fn width(&self) -> u32 {
        let (w, _) = self.size();
        w
    }

    fn height(&self) -> u32 {
        let (_, h) = self.size();
        h
    }

    fn view_mut(&mut self, x: u32, y: u32, width: u32, height: u32) -> SubSpriteRenderingContext<&mut Self::Context> {
        SubSpriteRenderingContext::new(self.inner_mut(), x, y, width, height)
    }

    fn draw_sprite(&mut self, coords: (u32, u32), sprite: &Texture) {
        let image = Image::new();

        let (xcoord, ycoord) = coords;

        // get the settings from here somehow
        // let sprite_size = self.inner_mut().sprite_renderer.settings.sprite_size;

        // let transform = self
        //     .inner_mut()
        //     .context
        //     .transform
        //     .trans_pos([(sprite_size * xcoord) as f64, (sprite_size * ycoord) as f64]);
    }
}

pub struct SpriteRendererSettings {
    size: (u32, u32),
    sprite_size: u32,
}

pub struct SpriteRenderer {
    gl: GlGraphics,
    settings: SpriteRendererSettings,
}

pub struct SpriteRenderingContext<'a> {
    sprite_renderer: &'a mut SpriteRenderer,
    context: Context,
}

pub struct SubSpriteRenderingContext<C> {
    context: C,
    x_offset: u32,
    y_offset: u32,
    width: u32,
    height: u32,
}

impl SpriteRendererSettings {
    pub fn new(size: (u32, u32), sprite_size: u32) -> Self {
        SpriteRendererSettings { size, sprite_size }
    }
}

impl SpriteRenderer {
    pub fn new(opengl: OpenGL, settings: SpriteRendererSettings) -> Self {
        let gl = GlGraphics::new(opengl);
        SpriteRenderer { gl, settings }
    }

    pub fn draw<F, U>(&mut self, viewport: Viewport, f: F)
    where
        F: FnOnce(&mut SpriteRenderingContext) -> U,
    {
        let context = self.gl.draw_begin(viewport);

        let mut sprite_rendering_context = SpriteRenderingContext::new(self, context);

        f(&mut sprite_rendering_context);

        self.gl.draw_end();
    }
}

impl<'a> SpriteRenderingContext<'a> {
    pub fn new(sprite_renderer: &'a mut SpriteRenderer, context: Context) -> Self {
        SpriteRenderingContext {
            sprite_renderer,
            context,
        }
    }

    pub fn clear(&mut self, color: graphics::types::Color) {
        graphics::clear(color, &mut self.sprite_renderer.gl);
    }
}

impl<'a> GenericContext for SpriteRenderingContext<'a> {
    type Context = Self;

    fn inner_mut(&mut self) -> &mut Self::Context {
        self
    }

    fn size(&self) -> (u32, u32) {
        self.sprite_renderer.settings.size
    }
}

impl<C> SubSpriteRenderingContext<C> {
    pub fn new(
        context: C,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
    ) -> Self {
        SubSpriteRenderingContext {
            context,
            x_offset,
            y_offset,
            width,
            height,
        }
    }
}

impl<C> GenericContext for SubSpriteRenderingContext<C>
where C: DerefMut,
C::Target: GenericContext + Sized,
 {
    type Context = C::Target;
    
    fn inner_mut(&mut self) -> &mut Self::Context {
        &mut self.context
    }

    fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
