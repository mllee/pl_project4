struct Sudoku{
	//board:[mut [mut 0u8, ..9], ..9],
	//board: [[u8;9];9]
	board: Vec<Vec<u8>>
	// TODO: Im not sure that this works. See comments in the constructors
}

fn main() {
	print!("main\n");
	let mut test = Sudoku::new();
	test.board[0] = vec![0,0,0,0,0,0,0,0,0];
	test.board[1] = vec![0,0,0,0,0,0,0,0,0];
	test.board[2] = vec![0,0,0,0,0,0,0,0,0];
	test.board[3] = vec![0,0,0,1,2,3,0,0,0];
	test.board[4] = vec![0,0,0,9,4,5,0,0,0];
	test.board[5] = vec![0,0,0,8,7,6,0,0,0];
	test.board[6] = vec![0,0,0,0,0,0,0,0,0];
	test.board[7] = vec![0,0,0,0,0,0,0,3,0];
	test.board[8] = vec![0,0,0,0,0,0,0,3,0];
	if test.check_boxes(8,8) {
		print!("boxes pass")
	}
	else {
		print!("boxes fail")
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
		let mut vec1 = vector.clone();
		vec1.sort();
		vec1.retain(|&i|i != 0);
		let mut vec2 = vec1.clone();
		vec1.dedup();
		println!("{:?}",vec2.len());
		println!("{:?}",vec2.len());
		return vec1.len() == vec2.len();
	}

	//Helper method for is_valid
	//Checks if all rows in the board are valid
	fn check_row(&self) -> bool { 
		//for row in self.board.iter().cloned() {
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