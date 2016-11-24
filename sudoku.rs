struct Sudoku{
	board:[u8;9*9],
}

impl Sudoku {
	fn new() -> Sudoku{} 
	//I dont know how to make a constructor -matt

	//The main solving method
	fn solve() -> bool {}

	//Helper method for is_valid
	fn check_row() -> bool {}
	//Helper method for is_valid
	fn check_col() -> bool {}
	//Helper method for is_valid
	fn check_boxes() -> bool {}

	//Checks if board is valid.
	fn is_valid() -> bool {
		self.check_boxes() && self.check_col() && self.check_row()
	}

	//print out the solution
	fn print() {}
}