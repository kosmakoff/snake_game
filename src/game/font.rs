use crate::point::Point;
use crate::sprite_renderer::GenericContext;
use image::{DynamicImage, GenericImageView};
use opengl_graphics::{Texture, TextureSettings};

pub type Font = [Texture; 128];

pub fn get_font(image: &DynamicImage, image_row_length: u32, size: u32) -> Font {
    let texture_settings = &TextureSettings::new();

    unsafe {
        let mut textures: Font = std::mem::uninitialized();

        let mut counter = 0;
        for texture in &mut textures[..] {
            if counter == 0 {
                let empty_texture =
                    Texture::empty(texture_settings).expect("Failed to create the empty texture");
                std::ptr::write(texture, empty_texture);
            } else {
                let counter = counter - 1;
                let x = counter % image_row_length;
                let y = counter / image_row_length;

                let one_character_image = image.view(x * size, y * size, size, size).to_image();
                let character_texture = Texture::from_image(&one_character_image, texture_settings);

                std::ptr::write(texture, character_texture);
            }
            counter += 1;
        }

        textures
    }
}

fn draw_character<'a, C, P>(context: &'a mut C, character: char, position: P, font: &Font)
where
    C: GenericContext,
    P: Into<Point>,
{
    let ascii_code = character as u8;
    let character_sprite = &font[ascii_code as usize];
    context.draw_sprite(position, character_sprite);
}

pub fn draw_string<'a, C, S, P>(context: &'a mut C, string: S, position: P, font: &Font)
where
    C: GenericContext,
    S: AsRef<str>,
    P: Into<Point>,
{
    let position: Point = position.into();
    let string = string.as_ref().to_uppercase();
    for (index, character) in string.chars().enumerate() {
        let char_position = position.offset((index as i32, 0));
        draw_character(context, character, char_position, font);
    }
}
