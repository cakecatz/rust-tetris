
pub struct Board {
    pub state : [[i32; 12]; 23],
}
impl Board{
	pub fn new() -> Board {
		return Board{state : [[9,0,0,0,0,0,0,0,0,0,0,9],
							  [9,0,0,0,0,0,0,0,0,0,0,9],//this line is ghost
							  [9,0,0,0,0,0,0,0,0,0,0,9],
    	 				 	  [9,0,0,0,0,0,0,0,0,0,0,9],
 		   					  [9,0,0,0,0,0,0,0,0,0,0,9],
    						  [9,0,0,0,0,0,0,0,0,0,0,9],
    						  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    				  [9,0,0,0,0,0,0,0,0,0,0,9],
		    		 		  [9,0,0,0,0,0,0,0,0,0,0,9],
		    		  		  [9,0,0,0,0,0,0,0,0,0,0,9],
		    		 		  [9,0,0,0,0,0,0,0,0,0,0,9],
		    		 		  [9,0,0,0,0,0,0,0,0,0,0,9],
		    		 		  [9,0,0,0,0,0,0,0,0,0,0,9],
		    		 		  [9,9,9,9,9,9,9,9,9,9,9,9]]};
	}

	pub fn marge(&mut self, act: [[i32; 4]; 4] , baseX: usize, baseY: usize ) -> [[i32; 12]; 23]{
	//baseX and baseY are left top
		let mut i:i32;
		let mut j:i32;
		let mut flag = 0;

		let mut tempX:usize = baseX + 1;
		let mut tempY:usize = baseY + 2;
		for i in 0..4 {
			for j in 0..4 {
				if self.state[tempY+i][tempX+j] == 0 && act[i][j] >= 1 {
					flag += 1;
				}

			}
		}

		if flag == 4 {
			for i in 0..4 {
				for j in 0..4 {
					if act[i][j] >= 1{
						self.state[tempY + i][tempX + j] = act[i][j];
					}
				}
			}
		}

		return self.state;


	}

	pub fn deleteLine(&mut self) -> [[i32; 12]; 23]{
		let mut i:usize;
		let mut j:usize;
		let mut flag=0;
		print!("come");
		for i in 2..22 {
			for j in 1..11{
				if self.state[i][j] == 0 {
					flag = 1;
				}
			}

			if flag != 1{
				for j in 1..11{
					self.state[i][j] = 0;
				}
			}
			self.state = self.fallLine(i);
			flag = 0;
		}

		return self.state;
	}

	pub fn fallLine(&mut self, line:usize) -> [[i32; 12]; 23]{//input delete line number
		let mut i:usize;
		let mut j:usize;
		let mut temp:usize;
		let mut flag =0;
		if flag == 0{
			for temp in 0..line{
				for j in 1..11{
					self.state[line-temp][j] = self.state[line - temp-1][j];
					self.state[line - temp-1][j] = 0;
				}
			}
		}

		return self.state;
	}
}