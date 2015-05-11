extern crate piston;

use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::event::*;

use opengl_graphics::{
    GlGraphics,
    Texture,
};

use board::Board;
use mino::Mino;
use mino;

pub struct Focus {
    pub x: i32,
    pub y: i32,
}

pub struct App {
    pub focus: Focus,
    board: Board,
    currentMino: Mino,
    maxHeight: i32,
    maxWidth: i32,
}



impl App {
    pub fn new() -> App {
        let mut board = Board::new();
        let mut mino = mino::createMino('t');
        return App {
            focus: Focus {
                x: 5,
                y: 1,
            },
            board: board,
            currentMino: mino,
            maxHeight: 21,
            maxWidth: 11,
        };
    }

    pub fn key_press(&mut self, args: &Button) {

        if *args == Keyboard(Key::Right) {
            self.move_focus(1, 0);
        }

        if *args == Keyboard(Key::Left) {
            self.move_focus(-1, 0);
        }

        if *args == Keyboard(Key::Up) {
            self.change_focus('y', 21);
        }

        if *args == Keyboard(Key::Down) {
            self.move_focus(0, 1);
        }

        if *args == Keyboard(Key::Z) {
            self.currentMino.next(&self.focus);
        }

        if *args == Keyboard(Key::X) {
            self.currentMino.prev(&self.focus);
        }
    }

    fn move_focus(&mut self, x: i32, y: i32) {
        let mut addY = y;
        let mut addX = x;

        for i in 0..4 {
            if self.currentMino.minos[i][1] == self.maxHeight {
                addY = 0;
            }

            if (self.currentMino.minos[i][0] + x) <= 0 || (self.currentMino.minos[i][0] + x) >= self.maxWidth {
                addX = 0;
            }
        }

        self.focus = Focus { x:self.focus.x + addX , y:self.focus.y + addY };
    }

    fn change_focus(&mut self, dimension: char, pos: i32) {
        if dimension == 'x' {
            self.focus = Focus { x: pos , y: self.focus.y };
        } else if dimension == 'y' {
            self.focus = Focus { x: self.focus.x , y: pos };
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const GRAY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const YELLOW:  [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        gl.draw(args.viewport(), |c, gl|{
            clear(BLACK, gl);
            self.board.clearBoard();
            let cell = rectangle::square( 0 as f64,  0 as f64, 20 as f64);


            self.board.minoMarge(&mut self.currentMino, &self.focus);

            for y in 2..23{
                for x in 0..12{

                    let transform = c.transform.trans((x*21) as f64,  ((y-2)*21) as f64);

                    match self.board.state[y][x] {
                        9 => rectangle(GRAY, cell, transform, gl),
                        0 => rectangle(WHITE, cell, transform, gl),
                        1 => rectangle(GREEN, cell, transform, gl),
                        _ => {}
                    }
                }
            }

        });
    }
}