pub struct Mino {
	pub name: char,
	pub minos: Vec<[[i32; 4]; 4]>,
	pub state: usize,
}

impl Mino {
	pub fn next(&mut self) {
		if self.minos.len() > (self.state + 1) {
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
			self.state = self.minos.len() - 1;
		}
	}
}


pub fn createMino(minoType: char) -> Mino {
	match minoType {
		'i' => return Mino {
			name: 'i',
			minos: vec![[[0,0,0,0],[1,1,1,1],[0,0,0,0],[0,0,0,0]],
						[[0,1,0,0],[0,1,0,0],[0,1,0,0],[0,1,0,0]]],
			state: 0,
		},

		's' => return Mino {
			name: 's',
			minos: vec![[[0,1,1,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
						[[1,0,0,0],[1,1,0,0],[0,1,0,0],[0,0,0,0]]],
			state: 0,
		},

		'z' => return Mino {
			name: 'z',
			minos: vec![[[1,1,0,0],[0,1,1,0],[0,0,0,0],[0,0,0,0]],
						[[0,1,0,0],[1,1,0,0], [1,0,0,0],[0,0,0,0]]],
			state: 0,
		},

		't' => return Mino {
			name: 't',
			minos: vec![[[0,1,0,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
						[[0,1,0,0],[0,1,1,0],[0,1,0,0],[0,0,0,0]],
						[[0,0,0,0],[1,1,1,0],[0,1,0,0],[0,0,0,0]],
						[[0,1,0,0],[1,1,0,0],[0,1,0,0],[0,0,0,0]]],
			state: 0,
		},

		'o' => return Mino {
			name: 'o',
			minos: vec![[[0,1,1,0],[0,1,1,0],[0,0,0,0],[0,0,0,0]]],
			state: 0,
		},

		'j' => return Mino {
			name: 'j',
			minos: vec![[[0,0,0,0],[1,1,1,0],[0,0,1,0],[0,0,0,0]],
						[[0,1,0,0],[0,1,0,0],[1,1,0,0],[0,0,0,0]],
						[[1,0,0,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
						[[0,1,1,0],[0,1,0,0],[0,1,0,0],[0,0,0,0]]],
			state: 0,
		},

		'l' => return Mino {
			name: 'l',
			minos: vec![[[0,0,0,0],[1,1,1,0],[1,0,0,0],[0,0,0,0]],
						[[1,1,0,0],[0,1,0,0],[0,1,0,0],[0,0,0,0]],
						[[0,0,1,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
						[[0,1,0,0],[0,1,0,0],[0,1,1,0],[0,0,0,0]]],
			state: 0,
		},

		_   => return Mino {
			name: '_',
			minos: vec![[[0,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,0]]],
			state: 0,
		}

	};
}