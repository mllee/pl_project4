use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;

//main starts here
fn main() {
    let f = File::open("/Users/Mimi/Documents/School Things/College/Junior Year/Semester 1/CS3270 - Programming Languages/Homework/hw7/sudoku.txt").unwrap();
    
    let mut v = vec![vec![0;9];9];
    
    let file = BufReader::new(&f);
    
    let vv: Vec<Vec<i32>> = file.lines()
    .filter_map(
        |l| l.ok().map(
            |s| s.split_whitespace()
                 .filter_map(|word| word.parse().ok())
                 .collect()))
    .collect();
    
    v = vv;
    
    for i in 0..9 {
			for j in 0..9 {
				print!("{} ", v[i][j]);
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
    
    //for line in file.lines() {
    //    let l = line.unwrap();
    //    println!("{}", l);
    //} 
    
}
