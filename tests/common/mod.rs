use sudoku::Sudoku;

use std::fs;
use std::convert::TryInto;
use std::error::Error;


pub fn read_matrix_and_solution(number: i32) -> (Sudoku, Sudoku) {
    if number < 1 || number > 6 {
        panic!("Sudoku does not exist!");
    }

    let matrix = &format!("tests/assets/Sudoku{}.txt", number);
    let solution = &format!("tests/assets/Solution{}.txt", number);

    let matrix = read_matrix(matrix);
    let solution = read_matrix(solution);

    (matrix, solution)
}


fn read_matrix(filename: &str) -> Sudoku {

    let contents = read_file(filename).unwrap();

    Sudoku::new(contents)
}

fn read_file(filename: &str) -> Result<Vec<[usize; 3]>, Box<dyn Error>> {

    let contents = fs::read_to_string(filename).unwrap();

    let contents = contents.split("\n")
        .flat_map(|line| {
            line.split_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();
    
    let contents = contents.chunks_exact(3)
        .map(|arr| arr.try_into().unwrap())
        .collect();

    Ok( contents )
}