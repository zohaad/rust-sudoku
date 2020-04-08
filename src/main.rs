use std::env;
use std::fs;
use std::convert::TryInto;
use std::error::Error;

use sudoku::Matrix;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let contents = read_file(&args).unwrap();

    let matrix = Matrix::new(contents);

    println!("Matrix to solve:\n{}", matrix);
}

// implementation specific
// temporary function, should be completely moved to mod.rs
// TODO: error handling
fn read_file(args: &[String]) -> Result<Vec<[u8; 3]>, Box<dyn Error>> {
    if args.len() < 2 {
        // same as 
        // return Err("not enough arguments".into());
        return Err("not enough arguments")?;
    }

    let filename = &args[1];

    let contents = fs::read_to_string(filename).unwrap();

    let contents = contents.split("\n")
        .flat_map(|line| {
            line.split_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
        })
        .collect::<Vec<_>>();
    
    let contents = contents.chunks_exact(3)
        .map(|arr| arr.try_into().unwrap())
        .collect();

    Ok( contents )
}