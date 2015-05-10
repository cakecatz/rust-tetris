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
}