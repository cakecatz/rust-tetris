extern crate piston;

use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::event::*;

use opengl_graphics::{
    GlGraphics,
    Texture,
};

pub struct Focus {
    pub x: i32,
    pub y: i32,
}

pub struct App {
    pub focus: Focus,
}



impl App {
    pub fn new() -> App {
        return App {
            focus: Focus {
                x: 0,
                y: 0,
            },
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
            self.move_focus(0, -1);
        }

        if *args == Keyboard(Key::Down) {
            self.move_focus(0, 1);
        }
    }

    fn move_focus(&mut self, x: i32, y: i32) {
        self.focus = Focus { x:self.focus.x + x , y:self.focus.y + y };
    }

    fn change_focus(&mut self, x: i32, y: i32) {
        self.focus = Focus { x: x , y: y };
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {

    }
}