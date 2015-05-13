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
    pub fn clearBoard(&mut self) {
        for i in 2..22 {
            for j in 2..12 {
                if self.state[i][j] < 9 {
                    self.state[i][j] = 0;
                }
            }
        }
    }

    pub fn checkAttach(&mut self, mino: &mut Mino, focus: &Focus){
    	let mut currentX = mino.minos[1][0];
        let mut currentY = mino.minos[1][1];
        let mut flag = 0;
        let mut over: i32 =0;

        mino.getCoordinate(focus.x, focus.y+1);
        for i in 0..4{
/*            currentX = mino.minos[i][0];
            currentY = mino.minos[i][1];*/
	        if self.state[mino.minos[i][1] as usize][mino.minos[i][0] as usize] >= 9{
	        	flag = 1;

	        	let calY:i32 = mino.minos[i][1] - mino.minos[1][1];
                if calY.abs() > over.abs(){
                    over = mino.minos[i][1] - mino.minos[1][1];
                }
	        }
    	}
    	if flag == 1 {
    		mino.getCoordinate(focus.x, focus.y + over);
    		for i in 0..4{
    			self.state[mino.minos[i][1] as usize][mino.minos[i][0] as usize] = 10;
    			println!("x:{}, y:{}", mino.minos[i][0],mino.minos[i][1]);
    		}
    	}
    	else{
    		mino.getCoordinate(focus.x, focus.y-1);
    	}
    }

    pub fn minoMarge(&mut self, mino: &mut Mino, focus: &Focus) {
        let minoPosX = focus.x - 1;
        let mut  x = focus.x;
        let mut  y = focus.y;
		mino.getCoordinate(x,y);
        for i in 0..4 {
        	if self.state[(mino.minos[i][1]) as usize][(mino.minos[i][0]) as usize] == 0 {
                self.state[(mino.minos[i][1]) as usize][(mino.minos[i][0]) as usize] = 1;
            }
        }

    }

	pub fn deleteLine(&mut self) -> [[i32; 14]; 24]{
		let mut i:usize;
		let mut j:usize;
		let mut flag=0;
		for i in 2..22 {
			for j in 2..12{
				if self.state[i][j] == 0 {
					flag = 1;
				}
			}

			if flag != 1{
				for j in 2..12{
					self.state[i][j] = 0;
				}
			}
			self.state = self.fallLine(i);
			flag = 0;
		}

		return self.state;
	}

	pub fn fallLine(&mut self, line:usize) -> [[i32; 14]; 24]{//input delete line number
		let mut i:usize;
		let mut j:usize;
		let mut temp:usize;
		let mut flag =0;
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