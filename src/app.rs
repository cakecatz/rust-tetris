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
use clock::*;

pub struct Focus {
    pub x: i32,
    pub y: i32,
}

pub struct App {
    pub focus: Focus,
    board: Board,
    current_mino: Mino,
    max_height: i32,
    max_width: i32,
    clock: Clock,
    pub turn_frame: usize,
}



impl App {
    pub fn new() -> App {
        let mut board = Board::new();
        let mut mino = mino::create_rand_mino();
        let mut clock = Clock::new();
        return App {
            focus: Focus {
                x: 6,
                y: 1,
            },
            board: board,
            current_mino: mino,
            max_height: 21,
            max_width: 12,
            turn_frame: 120,
            clock: clock,
        };
    }

    pub fn key_press(&mut self, args: Button) {

        if args == Keyboard(Key::Right) {
            self.move_focus(1, 0);
        }

        if args == Keyboard(Key::Left) {
            self.move_focus(-1, 0);
        }

        if args == Keyboard(Key::Up) {
            let drop_position = self.get_attach_position();
            self.change_focus('y', drop_position);
            self.check_attach();
        }

        if args == Keyboard(Key::Down) {
            self.check_attach();
            self.move_focus(0, 1);
        }

        if args == Keyboard(Key::Z) {
            let prev_state = self.current_mino.state;
            self.current_mino.next(&self.focus);
            self.slip_check(prev_state);
        }

        if args == Keyboard(Key::X) {
            let prev_state = self.current_mino.state;
            self.current_mino.prev(&self.focus);
            self.slip_check(prev_state);
        }

        // for debug
        if args == Keyboard(Key::N) {
            self.next_turn();
        }
    }

    fn next_turn(&mut self) {
        self.focus = Focus { x:6, y:1 };
        self.current_mino = mino::create_rand_mino();
    }

    fn get_deep_position(&mut self) -> i32{
        let mut max_y = -99;
        let current_y = self.focus.y;
        for i in 0..4 {
            if ( max_y < self.current_mino.minos[i][1] - current_y) {
                max_y = self.current_mino.minos[i][1] - current_y;
            }
        }
        return self.max_height - max_y;
    }

    fn get_attach_position(&mut self) -> i32{
        for i in 0..22 {
            for j in 0..4 {
                if self.board.state[(self.current_mino.minos[j][1] +i )as usize][(self.current_mino.minos[j][0]) as usize] >= 9 {
                    if i == 0{
                        for k in 0..4{
                            self.board.state[(self.current_mino.minos[k][1] +i )as usize][(self.current_mino.minos[k][0]) as usize] = self.current_mino.color_num;
                        }
                        return self.focus.y;
                    }
                    else{
                        for k in 0..4{
                            self.board.state[(self.current_mino.minos[k][1] +i-1 )as usize][(self.current_mino.minos[k][0]) as usize] = self.current_mino.color_num;
                        }
                        return self.focus.y + i -1;
                    }
                }
            }
        }
        return self.focus.y;
    }

    fn move_focus(&mut self, x: i32, y: i32) {
        let mut add_y = y;
        let mut add_x = x;
        let mut flag = 0;
        self.focus.x += add_x;
        self.focus.y += add_y;
        self.current_mino.get_coordinate(self.focus.x, self.focus.y);

        for i in 0..4 {
            if self.board.state[self.current_mino.minos[i][1] as usize][self.current_mino.minos[i][0] as usize] >= 9{
               flag = 1;
            }
        }
        if flag == 1{
            self.focus.x -= add_x;
            self.focus.y -= add_y;
            add_y = 0;
            add_x = 0;
            self.current_mino.get_coordinate(self.focus.x , self.focus.y);
        }
        self.focus = Focus { x:self.focus.x , y:self.focus.y};
    }

    fn change_focus(&mut self, dimension: char, pos: i32) {
        if dimension == 'x' {
            self.focus = Focus { x: pos , y: self.focus.y };
        } else if dimension == 'y' {
            self.focus = Focus { x: self.focus.x , y: pos };
        }
    }

    fn slip_check(&mut self, prev_state:usize){
        let mut temp_x:i32 =0;
        let mut temp_y:i32 =0;

        for i in 0..4{
            //何かにかぶったとき
            if self.board.state[self.current_mino.minos[i][1] as usize][self.current_mino.minos[i][0] as usize] >= 9{                let cur_y:i32 = self.current_mino.minos[i][1] - self.current_mino.minos[1][1];
                if cur_y.abs() > temp_y.abs(){
                    temp_y = self.current_mino.minos[i][1] - self.current_mino.minos[1][1];
                }

                let cur_x:i32 = self.current_mino.minos[i][0] - self.current_mino.minos[1][0];
                if cur_x.abs() > temp_x.abs(){
                    temp_x = self.current_mino.minos[1][0]
                          - self.current_mino.minos[i][0];
                }
            }
        }
        if temp_x !=0 || temp_y != 0 {
            println!("{},\n{}",temp_x,temp_y );
            if temp_y.abs() == 2{
                self.y_slip_modify(temp_y, prev_state);
            }
            else{
                self.x_slip_modify(temp_x, prev_state);
            }
        }

    }

    fn x_slip_modify(&mut self, temp_x:i32, prev_state:usize){
        let mut flag=0;
        let mut current_x = self.current_mino.minos[1][0]  + temp_x;
        let mut current_y = self.current_mino.minos[1][1];

        self.current_mino.get_coordinate(current_x, current_y);
        for i in 0..4{
            current_x = self.current_mino.minos[i][0];
            current_y = self.current_mino.minos[i][1];
            if self.board.state[current_y as usize][current_x as usize] >= 9{
                flag = 1;
            }
        }
        if flag != 1{
            self.focus.x += temp_x;  //
        }
        else{//横移動できなかったら
            current_x = self.current_mino.minos[1][0];
            current_y = self.current_mino.minos[1][1]-1;
            self.current_mino.get_coordinate( current_x, current_y);
            for i in 0..4{
                current_x = self.current_mino.minos[i][0];
                current_y = self.current_mino.minos[i][1];
                if self.board.state[current_y as usize][current_x as usize] >= 9{
                    flag = 2;
                }
            }
            if flag !=2 {
                self.focus.x += temp_x;
                self.focus.y -= 1;
            }
            else{//上にも行けなかったら
                current_x = self.current_mino.minos[1][0] - temp_x;
                current_y = self.current_mino.minos[1][1] + 1;
                self.current_mino.state = prev_state;
                self.current_mino.get_coordinate(current_x , current_y);


            }
        }

    }

    fn y_slip_modify(&mut self, temp_y:i32, prev_state:usize){
        let mut flag=0;
        let mut current_x = self.current_mino.minos[1][0];
        let mut current_y = self.current_mino.minos[1][1] + temp_y;

        self.current_mino.get_coordinate(current_x, current_y);
        for i in 0..4{
            current_x = self.current_mino.minos[i][0];
            current_y = self.current_mino.minos[i][1];
            if self.board.state[current_y as usize][current_x as usize] >= 9{
                flag = 1;
            }
        }
        if flag != 1{
            self.focus.y += temp_y;  //
        }
        else{//たて移動できなかったら
            current_x = self.current_mino.minos[1][0];
            current_y = self.current_mino.minos[1][1]  - temp_y;
            self.current_mino.state = prev_state;
            self.current_mino.get_coordinate(current_x, current_y);

        }

    }

pub fn check_attach(&mut self){
        let mut flag = 0;
        let mut over: i32 =0;
        self.current_mino.get_coordinate(self.focus.x, self.focus.y);
        for i in 0..4{
                if self.board.state[self.current_mino.minos[i][1] as usize][self.current_mino.minos[i][0] as usize] >= 9{
                    flag = 1;
                }
            }

        if flag == 1{
            self.current_mino.get_coordinate(self.focus.x, self.focus.y-1);
            for i in 0..4{
                if self.board.state[self.current_mino.minos[i][1] as usize][self.current_mino.minos[i][0] as usize] >= 9{
                    flag = 2;
                }
            }
        }
        if flag == 2{
            self.current_mino.get_coordinate(self.focus.x, self.focus.y-2);
        }

        if flag == 0 {
            self.current_mino.get_coordinate(self.focus.x, self.focus.y+1);
            for i in 0..4{
                if self.board.state[self.current_mino.minos[i][1] as usize][self.current_mino.minos[i][0] as usize] >= 9{
                    flag = 3;
                }
            }
            if flag == 3 {
                self.current_mino.get_coordinate(self.focus.x, self.focus.y);
            }
        }
        if flag == 2 || flag ==3{
            for i in 0..4{
                self.board.state[self.current_mino.minos[i][1] as usize][self.current_mino.minos[i][0] as usize] += 10;
                //create new mino
            }
            self.board.delete_line();
            self.next_turn();
        }
        self.current_mino.get_coordinate(self.focus.x, self.focus.y);

    }

    pub fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        use color::*;

        let WHITE: [f32; 4] = hex_color("ffffff");
        let TURQUOISE: [f32; 4] = hex_color("1abc9c");
        let EMERALD: [f32; 4] = hex_color("2ecc71");
        let PETER_RIVER: [f32; 4] = hex_color("3498db");
        let AMETHYST: [f32; 4] = hex_color("9b59b6");
        let SUN_FLOWER: [f32; 4] = hex_color("f1c40f");
        let CARROT: [f32; 4] = hex_color("e67e22");
        let ALIZARIN: [f32; 4] = hex_color("e74c3c");
        let GRAY:  [f32; 4] = hex_color("95a5a6");
        let WET_ASPHALT: [f32; 4] = hex_color("34495e");


        gl.draw(args.viewport(), |c, gl|{
            clear(WET_ASPHALT, gl);
            self.board.clear_board();
            let cell = rectangle::square( 0 as f64,  0 as f64, 20 as f64);


            self.board.mino_marge(&mut self.current_mino, &self.focus);

            for y in 2..23{
                for x in 1..13{

                    let transform = c.transform.trans(((x-1)*21) as f64,  ((y-2)*21) as f64);
                    let mut col = self.board.state[y][x];
                    if self.board.state[y][x]>10{
                        col -= 10;
                    }

                    match col {
                        9 => rectangle(GRAY, cell, transform, gl),
                        0 => rectangle(WHITE, cell, transform, gl),
                        1 => rectangle(TURQUOISE, cell, transform, gl),
                        2 => rectangle(EMERALD, cell, transform, gl),
                        3 => rectangle(PETER_RIVER, cell, transform, gl),
                        4 => rectangle(AMETHYST, cell, transform, gl),
                        5 => rectangle(SUN_FLOWER, cell, transform, gl),
                        6 => rectangle(CARROT, cell, transform, gl),
                        7 => rectangle(ALIZARIN, cell, transform, gl),
                        10 => rectangle(WET_ASPHALT, cell, transform, gl),//please chege this mino's color
                        _ => {}
                    }
                }
            }

        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        if self.clock.check() {
            self.check_attach();
            self.move_focus(0, 1);
        }
    }
}