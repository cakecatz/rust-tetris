pub struct Mino {
	pub name: char,
	pub minos: Vec<[i32; 2]>,
	pub state: usize,
	pub len: usize,
}

impl Mino {
	pub fn next(&mut self) {
		if self.len > (self.state + 1) {
			self.state += 1;
		} else {
			self.state = 0;
		}
	}

	pub fn prev(&mut self) {
		println!("{}", self.state);
		if 0 < self.state {
			self.state -= 1;
		} else {
			self.state = self.len - 1;
		}
	}

	pub fn getCoordinate(&mut self, x:i32, y:i32){
		self.minos = 
		match self.name {
		'i' => match self.state{
						0 => vec![[x-1,y],[x,y],[x+1,y],[x+2,y]],
						1 => vec![[x,y-1],[x,y],[x,y+1],[x,y+2]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},

		's' => match self.state{
						0 => vec![[x+1,y],[x,y],[x,y+1],[x-1,y+1]],
						1 => vec![[x-1,y-1],[x,y],[x-1,y],[x,y+1]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},

		'z' => match self.state{
						0 => vec![[x-1,y],[x,y],[x,y+1],[x+1,y+1]],
						1 => vec![[x+1,y-1],[x,y],[x+1,y],[x-1,y+1]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},

		't' => match self.state{
						0 => vec![[x,y-1],[x,y],[x+1,y],[x-1,y]],
						1 => vec![[x,y-1],[x,y],[x+1,y],[x,y+1]],
						2 => vec![[x-1,y],[x,y],[x+1,y],[x,y+1]],
						3 => vec![[x,y-1],[x,y],[x-1,y],[x,y+1]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},

		'o' => match self.state{
						0 => vec![[x+1,y],[x,y],[x,y-1],[x+1,y-1]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},

		'j' => match self.state{
						0 => vec![[x-1,y],[x,y],[x+1,y],[x+1,y+1]],
						1 => vec![[x,y+1],[x,y],[x,y-1],[x+1,y-1]],
						2 => vec![[x-1,y],[x,y],[x+1,y],[x-1,y-1]],
						3 => vec![[x,y-1],[x,y],[x,y+1],[x+1,y-1]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},
				
		'l' => match self.state{
						0 => vec![[x-1,y],[x,y],[x+1,y],[x-1,y+1]],
						1 => vec![[x,y+1],[x,y],[x,y-1],[x-1,y-1]],
						2 => vec![[x-1,y],[x,y],[x+1,y],[x+1,y-1]],
						3 => vec![[x,y-1],[x,y],[x,y+1],[x+1,y+1]],
						_ => vec![[x,y],[x,y],[x,y],[x,y]]},

		_ => vec![[x,y],[x,y],[x,y],[x,y]],
		}


	}
}


pub fn createMino(minoType: char) -> Mino {
	match minoType {
		'i' => return Mino {
			name: 'i',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:2,
		},

		's' => return Mino {
			name: 's',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:2,
		},

		'z' => return Mino {
			name: 'z',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:2,
		},

		't' => return Mino {
			name: 't',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:4,
		},

		'o' => return Mino {
			name: 'o',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:1,
		},

		'j' => return Mino {
			name: 'j',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:4,
		},

		'l' => return Mino {
			name: 'l',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:4,
		},

		_   => return Mino {
			name: '_',
			minos: vec![[0,0],[0,0],[0,0],[0,0]],
			state: 0,
			len:0,
		}

	};


}