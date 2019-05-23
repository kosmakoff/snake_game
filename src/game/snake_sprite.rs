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
}

fn get_texture(image: &DynamicImage, x: u32, y: u32, size: u32) -> Texture {
    Texture::from_image(
        &image.view(x * size, y * size, size, size).to_image(),
        &TextureSettings::new(),
    )
}

fn get_textures(image: &mut DynamicImage, x: u32, y: u32, size: u32) -> [Texture; 4] {
    let cropped_image = &image.crop(x * size, y * size, size, size);
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
        let sprite_data = include_bytes!("..\\resources\\snake.png");

        let mut image = image::load_from_memory_with_format(sprite_data, image::ImageFormat::PNG)
            .expect("Failed to load the texture.");

        image = image.resize(
            image.width() * scale,
            image.height() * scale,
            FilterType::Nearest,
        );

        let texture_size = 8 * scale;

        SpriteData {
            brick: get_texture(&image, 0, 0, texture_size),
            snake_head: get_textures(&mut image, 1, 0, texture_size),
            snake_body: get_textures(&mut image, 2, 0, texture_size),
            snake_tail: get_textures(&mut image, 2, 1, texture_size),
            cherry: get_texture(&image, 0, 1, texture_size),
            apple: get_texture(&image, 1, 1, texture_size),
        }
    }
}
