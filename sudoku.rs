// Sudoku Solver in rust
// Programming Languages
// Written by Matt Lee, Richard (Max) Kerr, and Michelle Fang
// We did not receive unauthorized aid in completing this assignment


use std::fs::File;
use std::io::Read;
use std::io::BufReader; 
use std::io::BufRead;

struct Sudoku{
	board: Vec<Vec<u8>>
}

fn main() {
	print!("main\n");
	let mut test = Sudoku::new();
	test.load_board();
	test.solve(0);
	test.print();
}

impl Sudoku {
	fn new() -> Sudoku{
		Sudoku{
			board: vec![ vec![0;9];9]
		}
	} 
    
	//Loads the sudoku board from a text file into class variable
    fn load_board(&mut self) {
        let f = File::open("C:\\cs270\\rust\\pl_project4\\sudoku.txt").unwrap();
        let file = BufReader::new(&f);

        let vv: Vec<Vec<u8>> = file.lines()
        .filter_map(
            |l| l.ok().map(
                |s| s.split_whitespace()
                     .filter_map(|word| word.parse().ok())
                     .collect()))
        .collect();

        self.board = vv;
    }

	//The main solving method
	fn solve(&mut self,index:u32) -> bool {
		if index == 81 {
			return true;
		}
		let row:u32 = index/9;
		let col:u32 = index%9;
		if self.board[row as usize][col as usize] != 0 {
			return self.solve(index+1);
		} 
		for guess in 1..10 {
			self.board[row as usize][col as usize] = guess;
			if self.is_valid(row as usize,col as usize) && self.solve(index+1) {
				return true;
			}
		}
		self.board[row as usize][col as usize] = 0;
		return false;
	}

	//Given a vector of numbers, returns whether or not duplicates exist
	//by sorting and filtering out duplicates.
	fn check_array(&self, vector:Vec<u8>) -> bool {
		let mut vec1 = vector.clone();
		vec1.sort();
		vec1.retain(|&i|i != 0);
		let mut vec2 = vec1.clone();
		vec1.dedup();
		return vec1.len() == vec2.len();
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

	//prints out the board
	fn print(&self) {
     for i in 0..9 {
			for j in 0..9 {
				print!("{} ", self.board[i][j]);
                if j == 2 || j == 5 {
                    print!("| ");
                }
			}
            if i == 2 || i == 5 {
                println!("\n------+-------+------");
            }
            else{
                println!("");
            }
		}
    
    }
}