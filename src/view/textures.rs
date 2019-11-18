use opengl_graphics::{Texture, TextureSettings};
use std::collections::HashMap;
use std::path::PathBuf;
//use graphics::Text;


pub fn texture_creator(texture_settings: &TextureSettings) -> HashMap<String, Texture> {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let water_texture = create_texture_from_path(texture_settings, "water.png", &assets);
    let wall_texture = create_texture_from_path(texture_settings, "wall.png", &assets);
    let ground_texture = create_texture_from_path(texture_settings, "ground.png", &assets);
    let tank_texture = create_texture_from_path(texture_settings, "tank.png", &assets);

    let mut textures = HashMap::new();
    textures.insert("water".to_owned(), water_texture);
    textures.insert("wall".to_owned(), wall_texture);
    textures.insert("ground".to_owned(), ground_texture);
    textures.insert("tank".to_owned(), tank_texture);
    textures
}

fn create_texture_from_path(texture_settings: &TextureSettings, path: &str, assets: &PathBuf) -> Texture {
    let texture_path = assets.join(path);
    let error_message = format!("can't open {}", path);
    Texture::from_path(texture_path, texture_settings).expect(&error_message)
}


#[cfg(test)]
mod tests {

    use super::*;
    use graphics::ImageSize;
    use piston::window::WindowSettings;
    use glutin_window::GlutinWindow as Window;
    use opengl_graphics::{OpenGL, Filter, GlGraphics, TextureSettings};
    use piston_window::PistonWindow;


    const OPENGL:OpenGL =OpenGL::V3_2;
    fn init_gl_helper(gl:OpenGL)->WindowSettings{
        let  settings:WindowSettings =  WindowSettings::new("Tanks", [640, 640])
            .graphics_api(gl)
            .resizable(false)
            .exit_on_esc(true);
        settings
    }

    #[test]
    fn create_texture_from_path_test() {

        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let path = "water.png";
        let texture = create_texture_from_path(&texture_settings, path, &assets);
        assert_eq!(texture.get_size(), (20, 20));
    }

    #[test]
    #[should_panic(expected="can't open water_wrong.png")]
    fn create_texture_from_wrong_path_test() {
        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let path = "water_wrong.png";
        let _texture = create_texture_from_path(&texture_settings, path, &assets);
    }
}