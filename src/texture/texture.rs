use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

pub struct Textures<'a> {
    pub car_texture_red: Texture<'a>,
    pub car_texture_blue: Texture<'a>,
    pub car_texture_yellow: Texture<'a>,
}

impl<'a> Textures<'a> {
    pub fn load(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        Self {
            car_texture_red: texture_creator.load_texture("assets/car.png").unwrap(),
            car_texture_blue: texture_creator
                .load_texture("assets/blue_car.png")
                .unwrap(),
            car_texture_yellow: texture_creator
                .load_texture("assets/yellow_car.png")
                .unwrap(),
        }
    }

    pub fn as_hash_map(&'a self) -> HashMap<String, &'a Texture<'a>> {
        let mut textures = HashMap::new();
        textures.insert("car_texture_red".to_string(), &self.car_texture_red);
        textures.insert("car_texture_blue".to_string(), &self.car_texture_blue);
        textures.insert("car_texture_yellow".to_string(), &self.car_texture_yellow);
        textures
    }
}