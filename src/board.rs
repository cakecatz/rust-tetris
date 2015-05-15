use mino::Mino;
use app::Focus;

pub struct Board {
    pub state : [[i32; 14]; 24],
}
impl Board{
	pub fn new() -> Board {
		return Board{state : [[9,9,0,0,0,0,0,0,0,0,0,0,9,9],
							  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],//this line is ghost
							  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
    	 				 	  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
 		   					  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
    						  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
    						  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    				  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    		 		  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    		  		  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    		 		  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    		 		  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    		 		  [9,9,0,0,0,0,0,0,0,0,0,0,9,9],
		    		 		  [9,9,9,9,9,9,9,9,9,9,9,9,9,9],
		    		 		  [9,9,9,9,9,9,9,9,9,9,9,9,9,9]]};
	}

    // 固定されたミノのStateは9とかにする
    // それ以外のミノは操作中のミノだから、残像が残らないように綺麗にする
    pub fn clear_board(&mut self) {
        for i in 2..22 {
            for j in 2..12 {
                if self.state[i][j] < 9 {
                    self.state[i][j] = 0;
                }
            }
        }
    }

    pub fn mino_marge(&mut self, mino: &mut Mino, focus: &Focus) {
//        let mino_pos_x = focus.x - 1;
        let x = focus.x;
        let y = focus.y;
		mino.get_coordinate(x,y);
        for i in 0..4 {
        	if self.state[(mino.minos[i][1]) as usize][(mino.minos[i][0]) as usize] == 0 {
                self.state[(mino.minos[i][1]) as usize][(mino.minos[i][0]) as usize] = mino.color_num;
            }
        }

    }

	pub fn delete_line(&mut self) -> [[i32; 14]; 24]{
		let mut flag=0;
		for i in 2..22 {
			for j in 2..12{
				if self.state[i][j] == 0 {
					flag = 1;
				}
			}

			if flag == 0{
				for j in 2..12{
					self.state[i][j] = 0;
				}
				self.state = self.fall_line(i);
			}	
			flag = 0;
		}

		return self.state;
	}

	pub fn fall_line(&mut self, line:usize) -> [[i32; 14]; 24]{//input delete line number
		let  flag =0;
		if flag == 0{
			for temp in 0..line{
				for j in 2..12{
					self.state[line-temp][j] = self.state[line - temp-1][j];
					self.state[line - temp-1][j] = 0;
				}
			}
		}

		return self.state;
	}
}