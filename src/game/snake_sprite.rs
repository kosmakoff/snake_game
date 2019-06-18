use crate::game::font::get_font;
use crate::game::font::Font;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use opengl_graphics::{Texture, TextureSettings};

pub struct SpriteData {
    pub brick: Texture,
    pub snake_head: [Texture; 4],
    pub snake_body: [Texture; 4],
    pub snake_tail: [Texture; 4],
    pub cherry: Texture,
    pub apple: Texture,
    pub font: Font,
}

fn get_texture(image: &DynamicImage, x: u32, y: u32, size: u32) -> Texture {
    Texture::from_image(
        &image.view(x * size, y * size, size, size).to_image(),
        &TextureSettings::new(),
    )
}

fn get_textures(image: &DynamicImage, x: u32, y: u32, size: u32) -> [Texture; 4] {
    let cropped_image_buffer = image.view(x * size, y * size, size, size).to_image();
    let cropped_image = DynamicImage::ImageRgba8(cropped_image_buffer);

    let texture_settings = &TextureSettings::new();
    [
        Texture::from_image(&cropped_image.to_rgba(), texture_settings),
        Texture::from_image(&cropped_image.rotate90().to_rgba(), texture_settings),
        Texture::from_image(&cropped_image.rotate180().to_rgba(), texture_settings),
        Texture::from_image(&cropped_image.rotate270().to_rgba(), texture_settings),
    ]
}

impl SpriteData {
    pub fn new(scale: u32) -> Self {
        let snake_data = include_bytes!("../resources/snake.png");
        let font_data = include_bytes!("../resources/font.png");

        let mut snake_image =
            image::load_from_memory_with_format(snake_data, image::ImageFormat::PNG)
                .expect("Failed to load the texture.");

        let mut font_image =
            image::load_from_memory_with_format(font_data, image::ImageFormat::PNG)
                .expect("Failed to load the font.");

        snake_image = snake_image.resize(
            snake_image.width() * scale,
            snake_image.height() * scale,
            FilterType::Nearest,
        );

        font_image = font_image.resize(
            font_image.width() * scale,
            font_image.height() * scale,
            FilterType::Nearest,
        );

        let texture_size = 8 * scale;

        SpriteData {
            brick: get_texture(&snake_image, 0, 0, texture_size),
            snake_head: get_textures(&snake_image, 1, 0, texture_size),
            snake_body: get_textures(&snake_image, 2, 0, texture_size),
            snake_tail: get_textures(&snake_image, 2, 1, texture_size),
            cherry: get_texture(&snake_image, 0, 1, texture_size),
            apple: get_texture(&snake_image, 1, 1, texture_size),
            font: get_font(&font_image, 16, texture_size),
        }
    }
}
