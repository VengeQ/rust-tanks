mod textures;

use graphics::types::Color;
use crate::{FSIZE, CELL_COUNT};


use opengl_graphics::Texture;
use graphics::{Graphics, Context};
use crate::graphics::Transformed;

use piston_window::image;
use crate::controller::GameController;
use graphics::character::CharacterCache;
use crate::model::Cell;

pub use textures::texture_creator;
use std::collections::HashMap;

///Rendering settings
pub struct GameViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    water_texture: Texture,
    wall_texture: Texture,
    ground_texture: Texture,
}

impl GameViewSettings {
    pub fn new(board_size: f64, mut textures: HashMap<String,Texture>) -> Self {
        Self {
            position: [20.0, 20.0],
            size: board_size,
            background_color: [0.5, 0.5, 0.5, 1.0],
            water_texture: textures.remove("water").unwrap(),
            wall_texture: textures.remove("wall").unwrap(),
            ground_texture: textures.remove("ground").unwrap()
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

    pub fn draw<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, glyphs: &mut C, c: &Context, g: &mut G) {
        match controller.game_state {
            _ => { self.draw_progress(controller, glyphs, c, g) }
        };
    }


    fn draw_progress<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, _glyphs: &mut C, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};
        let settings = &self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        for y in 0..CELL_COUNT {
            for x in 0..CELL_COUNT {
                let x1 = settings.position[0] + FSIZE * x as f64;
                let y1 = settings.position[1] + FSIZE * y as f64;
                let _x2 = settings.position[0] + x1 + settings.size / CELL_COUNT as f64;
                let _y2 = settings.position[1] + y1 + settings.size / CELL_COUNT as f64;

                match controller.game.board()[x][y] {
                    (Cell::Clear,_) => {
                        image(&self.settings.ground_texture, c.transform.trans(x1, y1), g);
                    }
                    (Cell::Water,_) => {
                        image(&self.settings.water_texture, c.transform.trans(x1, y1), g);
                    }
                    (Cell::Wall,_) => {
                        image(&self.settings.wall_texture, c.transform.trans(x1, y1), g);
                    }
                };
            }
        }
        let cell_edge = Line::new([0.7, 0.7, 0.7, 0.1], 1.0);
        for i in 0..CELL_COUNT {
            let x = settings.position[0] + i as f64 / CELL_COUNT as f64 * settings.size;
            let y = settings.position[1] + i as f64 / CELL_COUNT as f64 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }
    }
}
