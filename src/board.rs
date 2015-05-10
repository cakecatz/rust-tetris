pub struct Board {
    pub state : [[i32; 10]; 20],
}
impl Board{
	pub fn new() -> Board {
		return Board{state : [[0,0,0,0,0,0,0,0,0,0],
    	 				 	  [0,0,0,0,0,0,0,0,0,0],
 		   					  [0,0,0,0,0,0,0,0,0,0],
    						  [0,0,0,0,0,0,0,0,0,0],
    						  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    				  [0,0,0,0,0,0,0,0,0,0],
		    		 		  [0,0,0,0,0,0,0,0,0,0],
		    		  		  [0,0,0,0,0,0,0,0,0,0],
		    		 		  [0,0,0,0,0,0,0,0,0,0],
		    		 		  [0,0,0,0,0,0,0,0,0,0],
		    		 		  [0,0,0,0,0,0,0,0,0,0]],};
	}

	pub fn marge(&mut self, act: [[i32; 4]; 4] , baseX: usize, baseY: usize ) -> [[i32; 10]; 20]{
	//baseX and baseY are left top
		let mut i:i32;
		let mut j:i32;
		let mut flag = 0;

		for i in 0..4 {
			for j in 0..4 {
				if self.state[baseX+i][baseY+j] == 0 && act[i][j] >= 1 {
					flag += 1;
				}

			}
		}

		if flag == 4 {
			for i in 0..4 {
				for j in 0..4 {
					if act[i][j] >= 1{
						self.state[baseX + i][baseY + j] = act[i][j];
					}
				}
			}
		}

		return self.state;


	}
}