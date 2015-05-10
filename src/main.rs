extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };


mod minoGenerator;
mod board;


fn main() {
    let opengl = OpenGL::_3_2;
    let window = Window::new(
        opengl,
        WindowSettings::new(
            "Tetris",
            [20 * 10 +9, 20 * 20 +19]
        )
        .exit_on_esc(true)
    );

    let mut gl = GlGraphics::new(opengl);
/*******************************************************/
    let mut array = minoGenerator::createMino('i');
    let mut test = board::Board::new();             //test space
    test.state = test.marge(array[0], 2,2);
/******************************************************/      

    for e in window.events() {
        if let Some(r) = e.render_args() {
            use graphics::*;
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            gl.draw(r.viewport(), |c, gl|{
                clear(BLACK, gl);
                let cell = rectangle::square( 0 as f64,  0 as f64, 20 as f64);
                

                for y in 0..20{
                    for x in 0..10{
                        
                        let transform = c.transform.trans((x*21) as f64,  (y*21) as f64);
                        if  test.state[y][x] != 0{
                            rectangle(GREEN, cell, transform, gl);
                        }
                        else{
                            rectangle(WHITE, cell, transform, gl);
                        }
                    }
                }
            });
        }
    }
}
