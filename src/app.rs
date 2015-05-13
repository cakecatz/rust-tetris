extern crate piston;

use piston::input::Button;
use piston::input::Button::Keyboard;
use piston::input::Key;
use piston::event::*;
use std::num;

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
                x: 6,
                y: 1,
            },
            board: board,
            currentMino: mino,
            maxHeight: 21,
            maxWidth: 12,
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
            self.checkAttach();
            
        }

        if *args == Keyboard(Key::Down) {
            
            self.checkAttach();
            self.move_focus(0, 1);
            
        }

        if *args == Keyboard(Key::Z) {
            let mut prevState = self.currentMino.state;
            self.currentMino.next(&self.focus);
            self.slipCheck(prevState);
        }

        if *args == Keyboard(Key::X) {
            let mut prevState = self.currentMino.state;
            self.currentMino.prev(&self.focus);
            self.slipCheck(prevState);
        }
    }

    fn move_focus(&mut self, x: i32, y: i32) {
        let mut addY = y;
        let mut addX = x;

        for i in 0..4 {
            if self.currentMino.minos[i][1] == self.maxHeight {
                addY = 0;
            }

            if (self.currentMino.minos[i][0] + x) <= 1 || (self.currentMino.minos[i][0] + x) >= self.maxWidth {
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

    fn slipCheck(&mut self, prevState:usize){
        let mut tempX:i32 =0;
        let mut tempY:i32 =0;

        for i in 0..4{
            if self.board.state[self.currentMino.minos[i][1] as usize][self.currentMino.minos[i][0] as usize] >= 9{//何かにかぶったとき
                let calY:i32 = self.currentMino.minos[i][1] - self.currentMino.minos[1][1];
                if calY.abs() > tempY.abs(){
                    tempY = self.currentMino.minos[i][1] - self.currentMino.minos[1][1];
                }

                let calX:i32 = self.currentMino.minos[i][0] - self.currentMino.minos[1][0];
                if calX.abs() > tempX.abs(){
                    tempX = self.currentMino.minos[1][0]
                          - self.currentMino.minos[i][0];
                }
            }
        }
        if tempX !=0 || tempY != 0 {
            println!("{},\n{}",tempX,tempY );
            if tempY.abs() == 2{
                self.YslipModify(tempY, prevState);
            }
            else{
                self.XslipModify(tempX, prevState);
            }
        }

    }

    fn XslipModify(&mut self, tempX:i32, prevState:usize){
        let mut flag=0;
        let mut currentX = self.currentMino.minos[1][0]  + tempX;
        let mut currentY = self.currentMino.minos[1][1];

        self.currentMino.getCoordinate(currentX, currentY);
        for i in 0..4{
            currentX = self.currentMino.minos[i][0];
            currentY = self.currentMino.minos[i][1];
            if self.board.state[currentY as usize][currentX as usize] >= 9{
                flag = 1;
            }
        }
        if flag != 1{
            self.focus.x += tempX;  //
        }
        else{//横移動できなかったら
            currentX = self.currentMino.minos[1][0];
            currentY = self.currentMino.minos[1][1]-1;
            self.currentMino.getCoordinate( currentX, currentY);
            for i in 0..4{
                currentX = self.currentMino.minos[i][0];
                currentY = self.currentMino.minos[i][1];
                if self.board.state[currentY as usize][currentX as usize] >= 9{
                    flag = 2;
                }
            }
            if flag !=2 {
                self.focus.x += tempX;
                self.focus.y -= 1;
            }
            else{//上にも行けなかったら
                currentX = self.currentMino.minos[1][0] - tempX;
                currentY = self.currentMino.minos[1][1] + 1;
                self.currentMino.state = prevState;
                self.currentMino.getCoordinate(currentX , currentY);
                

            }
        }

    }

    fn YslipModify(&mut self, tempY:i32, prevState:usize){
        let mut flag=0;
        let mut currentX = self.currentMino.minos[1][0];
        let mut currentY = self.currentMino.minos[1][1] + tempY;

        self.currentMino.getCoordinate(currentX, currentY);
        for i in 0..4{
            currentX = self.currentMino.minos[i][0];
            currentY = self.currentMino.minos[i][1];
            if self.board.state[currentY as usize][currentX as usize] >= 9{
                flag = 1;
            }
        }
        if flag != 1{
            self.focus.y += tempY;  //
        }
        else{//たて移動できなかったら
            currentX = self.currentMino.minos[1][0];
            currentY = self.currentMino.minos[1][1]  - tempY;
            self.currentMino.state = prevState;
            self.currentMino.getCoordinate(currentX, currentY);
            
        }

    }

    pub fn checkAttach(&mut self){
        let mut currentX = self.currentMino.minos[1][0];
        let mut currentY = self.currentMino.minos[1][1];
        let mut flag = 0;
        let mut over: i32 =0;

        self.currentMino.getCoordinate(self.focus.x, self.focus.y+1);
        for i in 0..4{
            if self.board.state[self.currentMino.minos[i][1] as usize][self.currentMino.minos[i][0] as usize] >= 9{
                flag = 1;

                let calY:i32 = self.currentMino.minos[i][1] - self.currentMino.minos[1][1];
                if calY.abs() > over.abs(){
                    over = self.currentMino.minos[i][1] - self.currentMino.minos[1][1];
                }
            }
        }
        if flag == 1 {
            self.currentMino.getCoordinate(self.focus.x, self.focus.y + over);
            for i in 0..4{
                self.board.state[self.currentMino.minos[i][1] as usize][self.currentMino.minos[i][0] as usize] = 10;
                println!("x:{}, y:{}", self.currentMino.minos[i][0],self.currentMino.minos[i][1]);

                //create new mino
            }
        }
        else{
            self.currentMino.getCoordinate(self.focus.x, self.focus.y-1);
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
                for x in 1..13{

                    let transform = c.transform.trans(((x-1)*21) as f64,  ((y-2)*21) as f64);

                    match self.board.state[y][x] {
                        9 => rectangle(GRAY, cell, transform, gl),
                        0 => rectangle(WHITE, cell, transform, gl),
                        1 => rectangle(GREEN, cell, transform, gl),
                        10 => rectangle(YELLOW, cell, transform, gl),//please chege this mino's color
                        _ => {}
                    }
                }
            }

        });
    }
}