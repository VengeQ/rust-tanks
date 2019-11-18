use opengl_graphics::{Texture, TextureSettings};
use std::collections::HashMap;


pub fn texture_creator(texture_settings: &TextureSettings) -> HashMap<String, Texture> {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let water_texture_path = assets.join("water.png");
    let water_texture = Texture::from_path(
        water_texture_path,
        texture_settings,
    ).unwrap();
    let wall_texture_path = assets.join("wall.png");
    let wall_texture = Texture::from_path(
        wall_texture_path,
        texture_settings,
    ).unwrap();
    let ground_texture_path = assets.join("ground.png");
    let ground_texture = Texture::from_path(
        ground_texture_path,
        texture_settings,
    ).unwrap();

    let mut textures = HashMap::new();
    textures.insert("water".to_owned(), water_texture);
    textures.insert("wall".to_owned(), wall_texture);
    textures.insert("ground".to_owned(), ground_texture);
    textures
}