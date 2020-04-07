use std::env;
use std::process;
use sudoku::Matrix;


fn main() {
    let args: Vec<String> = env::args().collect();

    let matrix = Matrix::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        
        process::exit(1);
    });
    
    println!("{}", matrix);
}
