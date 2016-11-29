struct Sudoku{
	//board:[mut [mut 0u8, ..9], ..9],
	//board: [[u8;9];9]
	board: Vec<Vec<u8>>
	// TODO: Im not sure that this works. See comments in the constructors
}

fn main() {
	print!("main");
	let mut test = Sudoku::new();
	for x in 0..9 {
		 test.board[x] = vec![1, 2, 0, 4, 5, 6, 7, 8, 9];
	}
	if test.check_row() {
		print!("rows pass")
	}
	else {
		print!("rows fail")
	}
}

impl Sudoku {
	//TODO: Fix initialization here
	//Struggling to put a mutable array of arrays here
	fn new() -> Sudoku{
		Sudoku{
			board: vec![ vec![0;9];9]
			//board:[[0;9],..9] 
		}
	} 

	//fn get(&self, row: usize, col: usize) -> int u8 {
	//}

	//The main solving method
	fn solve() -> bool {
		return true
	}

	//Given a vector of numbers, returns whether or not duplicates exist
	//by sorting and filtering out duplicates.
	//Dont talk to me about time complexity.
	fn check_array(&self, vector:Vec<u8>) -> bool {
		let mut vec = vector.clone();
		vec.sort();
		vec.retain(|&i|i != 0);
		vec.dedup();
		return vec.len() == vector.len()
	}

	//Helper method for is_valid
	//Checks if all rows in the board are valid
	fn check_row(&self) -> bool { 
		for row in self.board.iter().cloned() {
			if self.check_array(row) != true {
				return false;
			}
		}
		return true;
	}
	//Helper method for is_valid
	//Checks if all columns in the board are valid
	fn check_col(&self) -> bool {
		let mut buffer = Vec::new();
		for i in 0..9 {
			for j in 0..9 {
				buffer.push(self.board[j][i]);
			}
			if self.check_array(buffer.clone()) != true {
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
	fn check_boxes(&self,x:usize,y:usize) -> bool {
		let mut superrow = (x/3)*3;
		let mut supercol = (y/3)*3;
		let mut subbox = Vec::new();
		for i in 0..3 {
			for j in 0..3 {
				subbox.push(self.board[superrow+i][supercol+j]);
			}
		}
		return self.check_array(subbox);
	}



	//Checks if board is valid.
	fn is_valid(&self,x: usize, y:usize) -> bool {
		self.check_boxes(x,y) && self.check_col() && self.check_row()
	}

	//print out the solution
	fn print() {}
}