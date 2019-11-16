mod textures;

use graphics::types::Color;
use crate::{FSIZE, SIZE, CELL_COUNT};

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
    water_texture: Texture,
    wall_texture: Texture,
    ground_texture:Texture
}

impl GameViewSettings {
    pub fn new(board_size: f64, texture: (Texture, Texture, Texture)) -> Self {
        Self {
            position: [20.0, 20.0],
            size: board_size,
            background_color: [0.5, 0.5, 0.5, 1.0],
            water_color: [0.0, 0.0, 0.8, 1.0],
            clear_color: [1.0, 1.0, 1.0, 1.0],
            wall_color: [0.3, 0.1, 0.1, 1.0],
            water_texture: texture.0,
            wall_texture: texture.1,
            ground_texture:texture.2
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

    pub fn draw<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, glyphs: &mut C, c: &Context, g: &mut G, texture_settings: &TextureSettings) {
        match controller.game_state {
            _ => { self.draw_progress(controller, glyphs, c, g, texture_settings) }
        };
    }

    pub fn draw_images<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G, texture_settings: &TextureSettings) {
        let ref settings = self.settings;
        for y in 0..CELL_COUNT {
            for x in 0..CELL_COUNT {
                let x1 = settings.position[0] + x as f64 / CELL_COUNT as f64 * settings.size;
                let y1 = settings.position[1] + y as f64 / CELL_COUNT as f64* settings.size;
                let x2 = x1 + settings.size / CELL_COUNT as f64;
                let y2 = x2 + settings.size / CELL_COUNT as f64;
                match controller.game.board()[x][y] {
                    Cell::Water => {
                        println!(" water in (x,y):{} {}", x, y);
                        image(&self.settings.water_texture, c.transform.trans(x1, y1), g);
                    }

                    _ => {}
                };
            }
        }
    }

    fn draw_progress<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, glyphs: &mut C, c: &Context, g: &mut G, texture_settings: &TextureSettings) {
        use graphics::{Line, Rectangle};
        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        for y in 0..CELL_COUNT {
            for x in 0..CELL_COUNT {
                let x1 = settings.position[0] + FSIZE*x as f64;
                let y1 = settings.position[1] + FSIZE*y as f64;
                let x2 = x1 + settings.size / CELL_COUNT as f64;
                let y2 = x2 + settings.size / CELL_COUNT as f64;

                let cell_rect = [
                    x1, y1,
                    x2, y2,
                ];

                match controller.game.board()[x][y] {
                    Cell::Clear => {
                        image(&self.settings.ground_texture, c.transform.trans(x1, y1), g);
                    }
                    Cell::Water => {
                        image(&self.settings.water_texture, c.transform.trans(x1, y1), g);
                    }
                    Cell::Wall => {
                        image(&self.settings.wall_texture, c.transform.trans(x1, y1), g);
                    }
                };
            }
        }
    }
}
