use std::collections::HashMap;
use std::path::PathBuf;

use opengl_graphics::{Texture, TextureSettings};

use crate::model::{Area, GameObjectType};

//use graphics::Text;
macro_rules! map {
    ($($key:expr => $value:expr),*) =>{
        {
            #[allow(unused_mut)]
            let mut result=std::collections::HashMap::new();
            $(
                result.insert($key, $value);
            )*
            result
        }
    };
}

//Вероятно стоит создать отдельно TextureInitializer
pub struct Textures {
    textures: HashMap<String, Texture>
}

impl Textures {
    pub fn board_texture_from_cell(&self, cell: Area) -> &Texture {
        match cell {
            Area::Clear => self.textures.get("ground").expect("Can't find 'ground' in textures"),
        }
    }

    pub fn object_texture_from_cell(&self, object: GameObjectType) -> &Texture {
        match object {
            GameObjectType::Water => self.textures.get("water").expect("Can't find 'water' in textures"),
            GameObjectType::Wall => self.textures.get("wall").expect("Can't find 'wall' in textures"),
            GameObjectType::Live => self.textures.get("live").expect("Can't find 'live' in textures"),
        }
    }


    pub fn new(texture_settings: &TextureSettings) -> Self {
        Self { textures: Textures::create_textures(texture_settings) }
    }

    pub fn get(&self, key: &str) -> &Texture {
        &self.textures.get(key).unwrap_or_else(|| panic!("Can't find `{}` texture", key))
    }

    fn create_textures(texture_settings: &TextureSettings) -> HashMap<String, Texture> {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();

        let water_texture = Textures::create_texture_from_path(texture_settings, "water.png", &assets);
        let wall_texture = Textures::create_texture_from_path(texture_settings, "wall.png", &assets);
        let ground_texture = Textures::create_texture_from_path(texture_settings, "ground.png", &assets);
        let tank_texture = Textures::create_texture_from_path(texture_settings, "tank.png", &assets);
        let heart_texture = Textures::create_texture_from_path(texture_settings, "heart.png", &assets);

        map!["water".to_owned() => water_texture, "wall".to_owned() => wall_texture,
            "ground".to_owned() => ground_texture, "tank".to_owned() => tank_texture,
            "heart".to_owned() => heart_texture]
    }

    fn create_texture_from_path(texture_settings: &TextureSettings, path: &str, assets: &PathBuf) -> Texture {
        let texture_path = assets.join(path);
        let error_message = format!("can't open {}", path);
        Texture::from_path(texture_path, texture_settings).expect(&error_message)
    }
}

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
        let mut x: HashMap<usize, String> = map![];
        assert_eq!(x, HashMap::<usize, String>::new());
        x.insert(1, "one".to_owned());

        let y = map![1 => "one".to_owned()];
        assert_eq!(y, x);
    }

    #[test]
    fn get_test(){
        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let textures =Textures::new(&texture_settings);
        textures.get("water");
    }

    #[test]
    #[should_panic(expected = "Can't find `test` texture")]
    fn get_test_wrong(){
        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let textures =Textures::new(&texture_settings);
        textures.get("test");
    }

    #[test]
    fn board_texture_from_cell_smoke_test(){
        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);

        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let textures = Textures::new(&texture_settings);
        textures.board_texture_from_cell(Area::Clear);
    }

    #[test]
    fn object_texture_from_cell_smoke_test(){
        let settings = init_gl_helper(OPENGL);
        let _window: PistonWindow<Window> = settings.build().expect("Could not create window");
        let _gl = GlGraphics::new(OPENGL);

        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let textures = Textures::new(&texture_settings);
        textures.object_texture_from_cell(GameObjectType::Water);
    }
}