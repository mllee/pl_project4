struct Sudoku{
	board:[mut [mut u8;9], ..9],
}

impl Sudoku {
	fn new() -> Sudoku{
		Sudoku{
			board:[[0;9],9] // Representing the board with a long ass array
		}
	} 
	//I dont know how to make a constructor -matt 

	//fn get(&self, row: usize, col: usize) -> int u8 {
	//}

	//The main solving method
	fn solve() -> bool {}

	//Helper method for is_valid
	//Checks if all rows in the board are valid
	fn check_row() -> bool { 
		for row in self.board {
			if check_array(row) != true {
				return false;
			}
		}
		return true;
	}
	//Helper method for is_valid
	//Checks if all columns in the board are valid
	fn check_col() -> bool {
		let mut buffer = Vec::new();
		for i in 0..9 {
			for j in 0..9 {
				buffer.push(board[j][i]);
			}
			if check_array(buffer) != true {
				return false;
			}
			buffer.truncate(0);
		}
		return true;
	}

	//Helper method for is_valid
	//Given an x,y position, checks if that subbox is valid
	//Note that unlike check row/col, this only checks the individual 
	//box, not the entire board
	fn check_boxes(x:usize,y:usize) -> bool {
		let mut superrow = (x/3)*3;
		let mut supercol = (y/3)*3;
		let mut box = Vec::new();
		for i in 0..3 {
			for j in 0..3 {
				box.push(board[superrow+i][supercol+j]);
			}
		}
		return check_array(box);
	}

	//Given a vector of numbers, returns whether or not duplicates exist
	//by sorting and filtering out duplicates.
	//Dont talk to me about time complexity.
	fn check_array(&self, vector) -> {
		let mut vec = vector.clone();
		vec.sort();
		vec.retain(|&i|i != 0);
		vec.dedup();
		return vec.len() == vector.len()
	}

	//Checks if board is valid.
	fn is_valid(x: usize, y:usize) -> bool {
		self.check_boxes() && self.check_col() && self.check_row(x,y);
	}

	//print out the solution
	fn print() {}
}