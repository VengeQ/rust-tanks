extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod controller;
mod model;
mod view;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};
use graphics::Rectangle;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Fifteen", [800, 600])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true);
    let mut window: Window = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new()); //not lazy.
    let mut gl = GlGraphics::new(opengl);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);

    let glyphs = &mut GlyphCache::new("assets/amazone.ttf", (), texture_settings)
        .expect("Could not load font from 'assets/amazone.ttf'");
    let mut game = Game::new();
    game.lvl1();
    let textures = crate::view::textures::Textures::new(&texture_settings);
    let mut game_controller = GameController::new(game);
    let mut game_view = GameView::new(GameViewSettings::new(CELL_COUNT as f64 * FSIZE, textures));


    let mut events = Events::new(EventSettings::new()); //not lazy.
    //event handler
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {

                use graphics::clear;

                graphics::clear([1.0; 4], g);
                game_view.draw(&mut game_controller, glyphs, &c, g);
            });
        }
    }
}



/*

 let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("tank.jpeg");
    let rust_logo = Texture::from_path(
        rust_logo,
        &texture_settings,
    ).unwrap();


                clear([1.0; 4], g);
                let board_rect = [
                    0.0, 0.0,
                    800.0, 600.0,
                ];
                Rectangle::new([0.5, 0.5, 0.5, 1.0])
                    .draw(board_rect, &c.draw_state, c.transform, g);
            });
        }
    }
}
