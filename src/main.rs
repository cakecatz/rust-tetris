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
/*******************************************************/
    let mut array = minoGenerator::createMino('i');
    let mut test = board::Board::new();             //test space
    test.state = test.marge(array[0], 0,0);
/******************************************************/

    for e in window.events() {
        if let Some(r) = e.render_args() {
            use graphics::*;
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
            const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
            const GRAY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
            const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];
            gl.draw(r.viewport(), |c, gl|{
                clear(BLACK, gl);
                let cell = rectangle::square( 0 as f64,  0 as f64, 20 as f64);


                for y in 2..23{
                    for x in 0..12{

                        let transform = c.transform.trans((x*21) as f64,  ((y-2)*21) as f64);

                        if  test.state[y][x] == 9 {
                            rectangle(GRAY, cell, transform, gl);
                        }
                        else if 0 == test.state[y][x]{
                            rectangle(WHITE, cell, transform, gl);
                        }
                        else{
                            rectangle(GREEN, cell, transform, gl);
                        }

                        if y as i32 == app.focus.y && x as i32 == app.focus.x {
                            rectangle(YELLOW, cell, transform, gl);
                        }
                    }
                }

            });
        }

        if let Some(ref args) = e.press_args() {
            app.key_press(args);
        }
    }
}
