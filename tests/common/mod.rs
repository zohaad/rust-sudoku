use sudoku::Matrix;

use std::fs;
use std::convert::TryInto;
use std::error::Error;


pub fn read_matrix_and_solution(number: i32) -> (Matrix, Matrix) {
    if number < 1 || number > 6 {
        panic!("Sudoku does not exist!");
    }

    let matrix = &format!("Sudoku{}.txt", number);
    let solution = &format!("Solution{}.txt", number);

    let matrix = read_matrix(matrix);
    let solution = read_matrix(solution);

    (matrix, solution)
}


fn read_matrix(filename: &str) -> Matrix {

    let contents = read_file(filename).unwrap();

    Matrix::new(contents)
}

fn read_file(filename: &str) -> Result<Vec<[u8; 3]>, Box<dyn Error>> {

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