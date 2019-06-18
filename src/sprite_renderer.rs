use crate::point::Point;
use crate::size::Size;
use graphics::{Context, Image, Transformed, Viewport};
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use std::ops::DerefMut;

pub trait GenericContext {
    type InnerContext: GenericContext;

    fn inner_mut(&mut self) -> &mut Self::InnerContext;

    fn context(&self) -> Context;

    fn settings(&self) -> &SpriteRendererSettings;

    fn graphics(&mut self) -> &mut GlGraphics;

    fn offset(&self) -> Point {
        (0, 0).into()
    }

    fn size(&self) -> Size;

    fn width(&self) -> u32 {
        self.size().width
    }

    fn height(&self) -> u32 {
        self.size().height
    }

    fn view_mut<P, S>(
        &mut self,
        offset: P,
        size: S,
    ) -> SubSpriteRenderingContext<&mut Self>
    where
        P: Into<Point>,
        S: Into<Size>,
        Self: Sized
    {
        SubSpriteRenderingContext::new(self, offset, size)
    }

    fn draw_sprite<P: Into<Point>>(&mut self, coords: P, sprite: &Texture) {
        let image = Image::new();

        let coords: Point = coords.into();

        // get the settings from here somehow
        let sprite_size = self.settings().sprite_size;
        let offset = self.offset();

        let context = self.context();

        let transform = context.transform.trans_pos([
            (sprite_size * (coords.x + offset.x) as u32) as f64,
            (sprite_size * (coords.y + offset.y) as u32) as f64,
        ]);

        image.draw(sprite, &context.draw_state, transform, self.graphics());
    }
}

pub struct SpriteRendererSettings {
    size: Size,
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
    inner_context: C,
    offset: Point,
    size: Size,
}

impl SpriteRendererSettings {
    pub fn new<S: Into<Size>>(size: S, sprite_size: u32) -> Self {
        SpriteRendererSettings {
            size: size.into(),
            sprite_size,
        }
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
    type InnerContext = SpriteRenderingContext<'a>;

    fn inner_mut(&mut self) -> &mut Self::InnerContext {
        self
    }

    fn context(&self) -> Context {
        self.context
    }

    fn settings(&self) -> &SpriteRendererSettings {
        &self.sprite_renderer.settings
    }

    fn graphics(&mut self) -> &mut GlGraphics {
        &mut self.sprite_renderer.gl
    }

    fn size(&self) -> Size {
        self.sprite_renderer.settings.size
    }
}

impl<C> SubSpriteRenderingContext<C>
where
    C: DerefMut,
    C::Target: GenericContext + Sized,
{
    pub fn new<P, S>(inner_context: C, offset: P, size: S) -> Self
    where
        P: Into<Point>,
        S: Into<Size>,
    {
        let offset: Point = offset.into();
        let size: Size = size.into();

        let inner_offset = inner_context.offset();

        SubSpriteRenderingContext {
            inner_context,
            offset: inner_offset.offset(offset),
            size: size,
        }
    }
}

impl<C> GenericContext for SubSpriteRenderingContext<C>
where
    C: DerefMut,
    C::Target: GenericContext + Sized,
{
    type InnerContext = C::Target;

    fn inner_mut(&mut self) -> &mut Self::InnerContext {
        &mut self.inner_context
    }

    fn context(&self) -> Context {
        self.inner_context.context()
    }

    fn settings(&self) -> &SpriteRendererSettings {
        self.inner_context.settings()
    }

    fn graphics(&mut self) -> &mut GlGraphics {
        self.inner_mut().graphics()
    }

    fn size(&self) -> Size {
        self.size
    }

    fn offset(&self) -> Point {
        self.offset
    }
}
