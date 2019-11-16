extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate find_folder;


pub use model::Game;

mod controller;
mod model;
mod view;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings, Texture};
use graphics::{Rectangle, CircleArc};
use crate::graphics::Transformed;

use std::fs::File;
use piston_window::{G2dTexture, Flip, PistonWindow, image};
use std::path::Path;
use graphics::types::Scalar;
use graphics::radians::Radians;
use crate::controller::GameController;
use crate::view::{GameView, GameViewSettings};
use std::borrow::Borrow;

pub const FSIZE: f64 = 20.0;
pub const SIZE: usize = 20;
pub const CELL_COUNT: usize = 30;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Tanks", [640, 640])
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true);
    let mut window: PistonWindow<Window> = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true)); //not lazy.
    let mut gl = GlGraphics::new(opengl);
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/amazone.ttf", (), texture_settings).expect("Could not load font");
    let mut game = Game::new();
    game.lvl1();
    let textures = crate::view::texture_creator(&texture_settings);
    let mut game_controller = GameController::new(game);
    let game_view = GameView::new(GameViewSettings::new(CELL_COUNT as f64 * FSIZE, textures)); //Сделать нормально

    //let pos top
    while let Some(e) = events.next(&mut window) {
        game_controller.event(game_view.settings.position, game_view.settings.size, &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;

                clear([1.0; 4], g);
                game_view.draw(&game_controller, glyphs, &c, g, &texture_settings);
                // game_view.draw_images(&game_controller, &c, g, &texture_settings);
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

 let board_rect = [
                    0.0, 0.0,
                    800.0, 600.0,
                ];
                Rectangle::new([0.5, 0.5, 0.5, 1.0])
                    .draw(board_rect, &c.draw_state, c.transform, g);
                CircleArc::new([1.0, 1.0, 1.0, 1.0], 15.0,
                               <Scalar as Radians>::_180() * 0.0,
                               <Scalar as Radians>::_180() * 1.0)
                    .draw([0.0, 0.0, 30.0, 30.0], &c.draw_state, c.transform.trans(250.0, 250.0), g);
                let transformed = if counter <= 100 {
                    counter += 1;
                    c.transform.trans(100.0 + counter as f64, 100.0).rot_deg(90.0)
                } else {
                    c.transform.trans(100.0 + counter as f64, 100.0).rot_deg(180.0)
                };

                image(&rust_logo, transformed, g);
                */