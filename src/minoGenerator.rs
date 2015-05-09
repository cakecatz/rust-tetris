pub fn createMino(minoType: char) -> Vec<[[u32; 4]; 4]>{
	let mut array = 
	match minoType{
		'i' => [[[0,0,0,0],
 				 [1,1,1,1],
				 [0,0,0,0],
				 [0,0,0,0],] ,

				 [0,1,0,0],
 				 [0,1,0,0],
				 [0,1,0,0],
				 [0,1,0,0],] ],

		's' => [[[0,1,1,0],
				 [1,1,0,0],
				 [0,0,0,0],
				 [0,0,0,0],],

				 [1,0,0,0],
				 [1,1,0,0],
				 [0,1,0,0],
				 [0,0,0,0],]},

		'z' => [[[1,1,0,0],
				 [0,1,1,0],
				 [0,0,0,0],
				 [0,0,0,0],],

				 [0,1,0,0],
				 [1,1,0,0],
				 [1,0,0,0],
				 [0,0,0,0],]],

		't' => [[[0,1,0,0],
				 [1,1,1,0],
				 [0,0,0,0],
				 [0,0,0,0],],

				 [0,1,0,0],
				 [0,1,1,0],
				 [0,1,0,0],
				 [0,0,0,0],],

				 [0,0,0,0],
				 [1,1,1,0],
				 [0,1,0,0],
				 [0,0,0,0],],

				 [0,1,0,0],
				 [1,1,0,0],
				 [0,1,0,0],
				 [0,0,0,0],]],

		'o' => [[[0,1,1,0],
				 [0,1,1,0],
				 [0,0,0,0],
				 [0,0,0,0],]],

		'j' => [[[0,0,0,0],
				 [1,1,1,0],
				 [0,0,1,0],
				 [0,0,0,0],],

				 [0,1,0,0],
				 [0,1,0,0],
				 [1,1,0,0],
				 [0,0,0,0],],

				 [1,0,0,0],
				 [1,1,1,0],
				 [0,0,0,0],
				 [0,0,0,0],],

				 [0,1,1,0],
				 [0,1,0,0],
				 [0,1,0,0],
				 [0,0,0,0],]},

		'l' => [[[0,0,0,0],
				 [1,1,1,0],
				 [1,0,0,0],
				 [0,0,0,0],],

				 [1,1,0,0],
				 [0,1,0,0],
				 [0,1,0,0],
				 [0,0,0,0],],

				 [0,0,1,0],
				 [1,1,1,0],
				 [0,0,0,0],
				 [0,0,0,0],],

				 [0,1,0,0],
				 [0,1,0,0],
				 [0,1,1,0],
				 [0,0,0,0],]},

		_   => [[0,0,0,0],
				[0,0,0,0],
				[0,0,0,0],
				[0,0,0,0],]],

	};
	return array;

}