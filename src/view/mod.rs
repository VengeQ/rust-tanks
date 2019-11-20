pub mod textures;
pub mod animator;

use graphics::types::Color;
use crate::{FSIZE, CELL_COUNT};

use opengl_graphics::Texture;
use graphics::{Graphics, Context};
use crate::graphics::Transformed;

use piston_window::image;
use crate::controller::GameController;
use graphics::character::CharacterCache;

use crate::view::textures::Textures;
use crate::model::Direction;
use graphics::math::Matrix2d;


///Rendering settings
pub struct GameViewSettings {
    position: [f64; 2],
    size: f64,
    background_color: Color,
    textures: Textures,
}

impl GameViewSettings {
    pub fn new(board_size: f64, textures: Textures) -> Self {
        Self {
            position: [20.0, 20.0],
            size: board_size,
            background_color: [0.5, 0.5, 0.5, 1.0],
            textures,
        }
    }
}


pub struct GameView {
    settings: GameViewSettings
}

impl GameView {
    pub fn new(settings: GameViewSettings) -> Self {
        Self { settings }
    }

    pub fn size(&self) -> f64 {
        self.settings.size
    }

    pub fn position(&self) -> [f64; 2] {
        self.settings.position
    }


    pub fn draw<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&mut self, controller: &mut GameController, glyphs: &mut C, c: &Context, g: &mut G) {
        match controller.game_state() {
            _ => {
                self.draw_game_in_progress(controller, glyphs, c, g)
            }
        };
    }
    //c:Context, g:Graphics, я не смог их вынести отсюда, нужно видимо передать их как мутабельные ссылки в  GameView  и в gl_draw в main,
    // но у меня не выходит
    fn draw_game_in_progress<G: Graphics<Texture=Texture>, C: CharacterCache<Texture=G::Texture>>(&mut self, controller: &mut GameController, _glyphs: &mut C, c: &Context, g: &mut G) {
        self.draw_board(c, g);
        self.draw_lvl(controller, c, g);
        self.draw_lines(c, g);
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
        let cell_edge = Line::new([0.4, 0.4, 0.4, 0.4], 0.56);
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
                let img = self.settings.textures.texture_from_cell(controller.gameboard_field([x, y]));

                image(img, c.transform.trans(x1, y1), g)
            }
        }
    }

    //player position
    fn draw_tank<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G) {
        let settings = &self.settings;
        use crate::model::Direction;
        let x1 = settings.position[0] + FSIZE * controller.player_location().0[0] as f64;
        let y1 = settings.position[1] + FSIZE * controller.player_location().0[1] as f64;
        let tank_texture = settings.textures.get("tank");


        let direction = controller.player_location().1;
        let transform = GameView::trans_with_rotate_by_direction([x1, y1], direction, c);
        //image(tank_texture,transform,g);

        match controller.player_location().1 {
            Direction::Top => image(tank_texture, c.transform.trans(x1, y1).rot_deg(0.0), g),
            Direction::Right => image(tank_texture, c.transform.trans(x1 + settings.position[0], y1).rot_deg(90.0), g),
            Direction::Bottom => image(tank_texture, c.transform.trans(x1 + settings.position[0], y1 + settings.position[1]).rot_deg(180.0), g),
            Direction::Left => image(tank_texture, c.transform.trans(x1, y1 + settings.position[1]).rot_deg(270.0), g),
        };
    }


    fn draw_lives<G: Graphics<Texture=Texture>>(&self, controller: &GameController, c: &Context, g: &mut G) {
        let settings = &self.settings;
        let (shift_x, shift_y) = (settings.size - settings.position[0], settings.size + settings.position[1]);
        let heart_texture = settings.textures.get("heart");
        for i in 0..3 {
            let heart_position_y = shift_y as f64;
            let heart_position_x = shift_x as f64 - settings.position[1] * i as f64;
            image(heart_texture, c.transform.trans(heart_position_x, heart_position_y), g);
        }
    }

    fn trans_with_rotate_by_direction(pos: [f64; 2], direction: Direction, c: &Context) -> Matrix2d {
        match direction {
            Direction::Top => c.transform.trans(pos[0], pos[1]).rot_deg(0.0),
            Direction::Right => c.transform.trans(pos[0] + FSIZE, pos[1]).rot_deg(90.0),
            Direction::Bottom => c.transform.trans(pos[0] + FSIZE, pos[1] + FSIZE).rot_deg(180.0),
            Direction::Left => c.transform.trans(pos[0], pos[1] + FSIZE).rot_deg(270.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eq_float_with_accuracy(number1: [[f64; 3]; 2], number2: [[f64; 3]; 2], accuracy: u32) -> bool {
        let dif = f64::from(10_u32.pow(accuracy));
        let row10 = number1[0];
        let row11 = number1[1];
        let row20 = number2[0];
        let row21 = number2[1];
        fn eq_float(x: f64, y: f64, dif: f64) -> bool {
            if x - y < 1.0 / dif {
                true
            } else {
                false
            }
        }


        eq_float(row10[0], row20[0], dif) & &eq_float(row10[1], row20[1], dif)
            & &eq_float(row10[2], row20[2], dif) & &eq_float(row11[0], row21[0], dif)
            & &eq_float(row11[1], row21[1], dif) & &eq_float(row11[2], row21[2], dif)
    }


    #[test]
    fn trans_with_rotate_by_direction_test() {
        let c = Context::new();
        let position = [20.0, 20.0];
        let sin0 = 0.0;
        let sin90 = 1.0;
        let sin180 = sin0;
        let sin270 = -1.0;
        let cos0 = sin90;
        let cos90 = sin0;
        let cos180 = -1.0;
        let cos270 = sin0;

        let transformed_top = GameView::trans_with_rotate_by_direction(position, Direction::Top, &c);
        assert_eq!(transformed_top, [[cos0, -sin0, position[0]], [sin0, cos0, position[1]]]);

        let transformed_bottom = GameView::trans_with_rotate_by_direction(position, Direction::Bottom, &c);
        let expected_bottom = [[cos180, -sin180, position[0] + FSIZE], [sin180, cos180, position[1] + FSIZE]];
        assert!(eq_float_with_accuracy(transformed_bottom, expected_bottom, 5));
    }
}