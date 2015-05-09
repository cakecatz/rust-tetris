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
            [20 * 10 +19, 20 * 20 +9]
        )
        .exit_on_esc(true)
    );

    let mut x = 0;
    let mut y = 0; 
    let mut gl = GlGraphics::new(opengl);


    for e in window.events() {
        if let Some(r) = e.render_args() {
            use graphics::*;
            const GREEN: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            const GRAY: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            gl.draw(r.viewport(), |c, gl|{
                clear(GREEN, gl);
                let cell = rectangle::square( 0 as f64,  0 as f64, 20 as f64);
                

                for x in 0..10{
                    for y in 0..20{
                        
                        let transform = c.transform.trans((x*21) as f64,  (y*21) as f64);
                        rectangle(GRAY, cell, transform, gl);
                    }
                }
            });
        }
    }
}
