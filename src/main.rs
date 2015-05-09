extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Tetris",
            [20 * 10, 20 * 20]
        )
        .exit_on_esc(true)
    );


    let mut gl = GlGraphics::new(opengl);


    for e in window.events() {
        if let Some(r) = e.render_args() {
            use graphics::*;
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            gl.draw(r.viewport(), |c, gl|{

                clear(GREEN, gl);
            });
        }
    }
}
