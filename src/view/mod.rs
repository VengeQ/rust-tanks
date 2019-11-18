mod textures;

use graphics::types::Color;
use crate::{FSIZE, CELL_COUNT};

use opengl_graphics::Texture;
use graphics::{Graphics, Context};
use crate::graphics::Transformed;

use piston_window::image;
use crate::controller::GameController;
use graphics::character::CharacterCache;
use crate::model::{Cell, Orientation};

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
    tank_texture: Texture,
}

impl GameViewSettings {
    pub fn new(board_size: f64, mut textures: HashMap<String, Texture>) -> Self {
        Self {
            position: [20.0, 20.0],
            size: board_size,
            background_color: [0.5, 0.5, 0.5, 1.0],
            water_texture: textures.remove("water").unwrap(),
            wall_texture: textures.remove("wall").unwrap(),
            ground_texture: textures.remove("ground").unwrap(),
            tank_texture: textures.remove("tank").unwrap(),
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
            _ => { self.draw_game_in_progress(controller, glyphs, c, g) }
        };
    }

    fn draw_game_in_progress<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, _glyphs: &mut C, c: &Context, g: &mut G) {
        let settings = &self.settings;
        self.draw_board(c, g, settings);
        self.draw_lvl(controller, c, g, settings);
        self.draw_lines(controller, c, g, settings);
        self.draw_tank(controller, c, g, settings);
    }

    //Draw separate elements.
    ///Todo 'draw_lines' and 'draw_board' calculation should be memoized.
    #[inline]
    fn draw_board<G: Graphics<Texture=Texture>>(&self, c: &Context, g: &mut G, settings: &GameViewSettings) {
        use graphics::Rectangle;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);
    }

    #[inline]
    fn draw_lines<G: Graphics<Texture=Texture>>(&self, _controller: &GameController, c: &Context, g: &mut G, settings: &GameViewSettings) {
        use graphics::Line;
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

    //current lvl board
    fn draw_lvl<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G, settings: &GameViewSettings) {
        for y in 0..CELL_COUNT {
            for x in 0..CELL_COUNT {
                let x1 = settings.position[0] + FSIZE * x as f64;
                let y1 = settings.position[1] + FSIZE * y as f64;

                match controller.game.board()[x][y] {
                    (Cell::Clear, _) => image(&self.settings.ground_texture, c.transform.trans(x1, y1), g),
                    (Cell::Water, _) => image(&self.settings.water_texture, c.transform.trans(x1, y1), g),
                    (Cell::Wall, _) => image(&self.settings.wall_texture, c.transform.trans(x1, y1), g),
                };
            }
        }
    }
    //player position
    fn draw_tank<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G, settings: &GameViewSettings) {
        let x1 = settings.position[0] + FSIZE * controller.position.0[0] as f64;
        let y1 = settings.position[1] + FSIZE * controller.position.0[1] as f64;
        match controller.position.1 {
            Orientation::Top => image(&settings.tank_texture, c.transform.trans(x1, y1).rot_deg(0.0), g),
            Orientation::Right => image(&settings.tank_texture, c.transform.trans(x1 + settings.position[0], y1).rot_deg(90.0), g),
            Orientation::Bottom => image(&settings.tank_texture, c.transform.trans(x1 + settings.position[0], y1 + settings.position[1]).rot_deg(180.0), g),
            Orientation::Left => image(&settings.tank_texture, c.transform.trans(x1, y1 + settings.position[1]).rot_deg(270.0), g),
        };
        // image(&settings.tank_texture, c.transform.trans(x1, y1), g);
        //   image(&settings.tank_texture, c.transform.trans(40.0,40.0).rot_deg(90.0), g);
    }
}
