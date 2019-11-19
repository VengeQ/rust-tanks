pub mod textures;

use graphics::types::Color;
use crate::{FSIZE, CELL_COUNT};

use opengl_graphics::Texture;
use graphics::{Graphics, Context};
use crate::graphics::Transformed;

use piston_window::image;
use crate::controller::GameController;
use graphics::character::CharacterCache;

use crate::view::textures::Textures;


///Rendering settings
pub struct GameViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    textures:Textures,

}

impl GameViewSettings {
    pub fn new(board_size: f64, textures: Textures) -> Self {
        Self {
            position: [20.0, 20.0],
            size: board_size,
            background_color: [0.5, 0.5, 0.5, 1.0],
            textures
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
        match controller.game_state() {
            _ => { self.draw_game_in_progress(controller, glyphs, c, g) }
        };
    }

    //c:Context, g:Graphics, я не смог их вынести отсюда, нужно видимо передать их как мутабельные ссылки в  GameView  и в gl_draw в main,
    // но у меня не выходит
    fn draw_game_in_progress<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&self, controller: &GameController, _glyphs: &mut C, c: &Context, g: &mut G) {
        self.draw_board(c, g);
        self.draw_lines(c, g);
        self.draw_lvl(controller, c, g);
        self.draw_tank(controller, c, g);
    }

    //Draw separate elements.
    ///Todo 'draw_lines' and 'draw_board' calculation should be memoized.
    #[inline]
    fn draw_board<G: Graphics<Texture=Texture>>(&self, c: &Context, g: &mut G) {
        let settings = &self.settings;
        use graphics::Rectangle;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);
    }
    #[inline]
    fn draw_lines<G: Graphics<Texture=Texture>>(&self, c: &Context, g: &mut G) {
        let settings = &self.settings;
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
    fn draw_lvl<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G) {
        let settings = &self.settings;
        for y in 0..CELL_COUNT {
            for x in 0..CELL_COUNT {
                let x1 = settings.position[0] + FSIZE * x as f64;
                let y1 = settings.position[1] + FSIZE * y as f64;
                let img = self.settings.textures.texture_from_cell(controller.gameboard_field([x,y]));

                image(img, c.transform.trans(x1, y1), g)

            }
        }
    }
    //player position
    fn draw_tank<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G) {
        let settings = &self.settings;
        use crate::model::Direction;
        let x1 = settings.position[0] + FSIZE * controller.location().0[0] as f64;
        let y1 = settings.position[1] + FSIZE * controller.location().0[1] as f64;
        let tank_texture = settings.textures.get("tank");
        match controller.location().1 {
            Direction::Top => image(tank_texture, c.transform.trans(x1, y1).rot_deg(0.0), g),
            Direction::Right => image(tank_texture, c.transform.trans(x1 + settings.position[0], y1).rot_deg(90.0), g),
            Direction::Bottom => image(tank_texture, c.transform.trans(x1 + settings.position[0], y1 + settings.position[1]).rot_deg(180.0), g),
            Direction::Left => image(tank_texture, c.transform.trans(x1, y1 + settings.position[1]).rot_deg(270.0), g),
        };

    }
}
