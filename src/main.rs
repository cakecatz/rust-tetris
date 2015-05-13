extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate time;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod mino;
mod board;
mod app;


fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Tetris",
            [20 * 12 +11, 20 * 21 +20]
        )
        .exit_on_esc(true)
    );

    let mut app = app::App::new();

    let mut gl = GlGraphics::new(opengl);

    let mut counter = 0;

    for e in window.events().ups(60) {

        counter += 1;

        if app.turnFrame <= counter {
            counter = 0;
        }

        if let Some(r) = e.render_args() {
            app.render(&r, &mut gl);
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }
    }
}
