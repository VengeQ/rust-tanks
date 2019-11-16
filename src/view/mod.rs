mod textures;

use graphics::types::Color;
use crate::{FSIZE, SIZE};

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings, Texture};
use graphics::{Rectangle, CircleArc, Graphics, Context};
use crate::graphics::Transformed;

use std::fs::File;
use piston_window::{G2dTexture, Flip, PistonWindow, image};
use std::path::Path;
use graphics::types::Scalar;
use graphics::radians::Radians;
use crate::controller::GameController;
use graphics::character::CharacterCache;
use crate::model::Cell;

pub use textures::texture_creator;

///Rendering settings
pub struct GameViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    water_color: Color,
    clear_color: Color,
    wall_color: Color,
    water_texture:Texture
}

impl GameViewSettings {
    pub fn new(board_size: f64, texture:Texture) -> Self {
        Self {
            position: [20.0, 20.0],
            size: FSIZE * board_size,
            background_color: [0.5, 0.5, 0.5, 1.0],
            water_color: [0.0, 0.0, 0.8, 1.0],
            clear_color: [1.0, 1.0, 1.0, 1.0],
            wall_color: [0.3, 0.1, 0.1, 1.0],
            water_texture:texture
        }
    }
}

pub struct GameView {
    pub settings: GameViewSettings
}

impl GameView {
    pub fn new(settings: GameViewSettings) -> Self {
        Self { settings }
    }

    pub fn draw<G: Graphics, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, glyphs: &mut C, c: &Context, g: &mut G, texture_settings: &TextureSettings) {
        match controller.game_state {
            _ => { self.draw_progress(controller, glyphs, c, g, texture_settings) }
        };
    }

    pub fn draw_images<G: Graphics<Texture = Texture>>(&self, controller: &GameController, c: &Context, g: &mut G, texture_settings: &TextureSettings) {
        let ref settings = self.settings;
        for y in 0..SIZE {
            for x in 0..SIZE {
                let x1 = settings.position[0] + x as f64 / FSIZE * settings.size;
                let y1 = settings.position[1] + y as f64 / FSIZE * settings.size;
                let x2 = x1 + settings.size / FSIZE;
                let y2 = x2 + settings.size / FSIZE;
                match controller.game.board()[x][y] {
                    Cell::Water => {
                        println!(" water in (x,y):{} {}",  x, y);
                        image(&self.settings.water_texture, c.transform.trans(x1,y1), g);
                    }

                    _ => {}
                };
            }
        }
    }

    fn draw_progress<G: Graphics, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, glyphs: &mut C, c: &Context, g: &mut G, texture_settings: &TextureSettings) {
        use graphics::{Line, Rectangle};
        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets").unwrap();
        let water_texture_path = assets.join("water.png");
        let water_texture = Texture::from_path(
            water_texture_path,
            texture_settings,
        ).unwrap();


        for y in 0..SIZE {
            for x in 0..SIZE {
                let x1 = settings.position[0] + x as f64 / FSIZE * settings.size;
                let y1 = settings.position[1] + y as f64 / FSIZE * settings.size;
                let x2 = x1 + settings.size / FSIZE;
                let y2 = x2 + settings.size / FSIZE;

                let cell_rect = [
                    x1, y1,
                    x2, y2,
                ];

                match controller.game.board()[x][y] {
                    Cell::Clear => {
                        //   println!("rect:{:?}   clear in (x,y):{} {}",cell_rect,x, y);
                        Rectangle::new(settings.clear_color)
                            .draw(cell_rect, &c.draw_state, c.transform, g);
                    }
                    Cell::Water => {
                    }
                    Cell::Wall => {
                        Rectangle::new(settings.wall_color)
                            .draw(cell_rect, &c.draw_state, c.transform, g);
                    }
                };
            }
        }
    }
}
