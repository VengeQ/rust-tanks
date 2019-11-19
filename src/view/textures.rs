use std::collections::HashMap;
use std::path::PathBuf;

use opengl_graphics::{Texture, TextureSettings};

use crate::model::{Cell, Direction};

//use graphics::Text;
macro_rules! map {
    ($($key:expr => $value:expr),*) =>{
        {
            let mut map=std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

//Вероятно стоит создать отдельно TextureInitializer
pub struct Textures {
    textures: HashMap<String, Texture>
}

impl Textures {
    pub fn texture_from_cell(&self, cell: (Cell, Direction)) -> &Texture {
        match cell {
            (Cell::Clear, _) => self.textures.get("ground").expect("Can't find 'ground' in textures"),
            (Cell::Water, _) => self.textures.get("water").expect("Can't find 'water' in textures"),
            (Cell::Wall, _) => self.textures.get("wall").expect("Can't find 'wall' in textures"),
        }
    }

    pub fn new(texture_settings: &TextureSettings) -> Self {
        Self { textures: Textures::create_textures(texture_settings) }
    }

    pub fn get(&self, key:&str) -> &Texture{
        &self.textures.get(key).unwrap_or_else(|| panic!("Can't find `{}` texture", key))
    }

    fn create_textures(texture_settings: &TextureSettings) -> HashMap<String, Texture> {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let water_texture = Textures::create_texture_from_path(texture_settings, "water.png", &assets);
        let wall_texture = Textures::create_texture_from_path(texture_settings, "wall.png", &assets);
        let ground_texture = Textures::create_texture_from_path(texture_settings, "ground.png", &assets);
        let tank_texture = Textures::create_texture_from_path(texture_settings, "tank.png", &assets);

        map!["water".to_owned() => water_texture,"wall".to_owned() => wall_texture,
         "ground".to_owned() => ground_texture,"tank".to_owned() => tank_texture]
    }
    fn create_texture_from_path(texture_settings: &TextureSettings, path: &str, assets: &PathBuf) -> Texture {
        let texture_path = assets.join(path);
        let error_message = format!("can't open {}", path);
        Texture::from_path(texture_path, texture_settings).expect(&error_message)
    }
}


///ToDo покрыть тестами!
#[cfg(test)]
mod tests {
    use glutin_window::GlutinWindow as Window;
    use graphics::ImageSize;
    use opengl_graphics::{Filter, GlGraphics, OpenGL, TextureSettings};
    use piston::window::WindowSettings;
    use piston_window::PistonWindow;

    use super::*;

    const OPENGL: OpenGL = OpenGL::V3_2;

    fn init_gl_helper(gl: OpenGL) -> WindowSettings {
        let settings: WindowSettings = WindowSettings::new("Tanks", [640, 640])
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
        let texture = Textures::create_texture_from_path(&texture_settings, path, &assets);
        assert_eq!(texture.get_size(), (20, 20)); //(20,20) - base size of texture.
    }

    #[test]
    #[should_panic(expected = "can't open water_wrong.png")]
    fn create_texture_from_wrong_path_test() {
        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);

        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let path = "water_wrong.png";
        let _texture = Textures::create_texture_from_path(&texture_settings, path, &assets);
    }

    #[test]
    fn map_macro_test() {
        let x: HashMap<String, String> = map![];
        assert_eq!(x, HashMap::<String, String>::new());
        let y1 = map![1 => "one".to_owned()];
        let mut y2 = HashMap::new();
        y2.insert(1, "one".to_owned());
        assert_eq!(y1, y2);
    }
}