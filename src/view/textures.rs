use opengl_graphics::{Texture,TextureSettings};

struct CustomTextures{

}

pub fn texture_creator(texture_settings:&TextureSettings) -> Texture{
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let water_texture_path = assets.join("water.png");
    let water_texture = Texture::from_path(
        water_texture_path,
        texture_settings,
    ).unwrap();

    water_texture
}