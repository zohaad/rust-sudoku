use std::ops::Index;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::slice;

#[derive(Clone, Copy)]
enum Cell {
    Candidates([bool; 9]),
    Solution(u8),
    NoSolution, // useful if Candidates are all false
    
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // use write!(f, "{}", something)
        // which returns a Result
        match self {
            Cell::Candidates(_) => write!(f, " "),
            Cell::Solution(x) => write!(f, "{}", x),
            Cell::NoSolution => write!(f, "?"),
        }
    }
}

// 9x9 grid of Cells
struct Matrix([[Cell; 9]; 9]);

impl Matrix {
    // TODO: implement file input
    fn new() -> Self {
        Matrix([[Cell::Candidates([true; 9]); 9]; 9])
    }
}

impl Index<usize> for Matrix {
    type Output = [Cell; 9];
    fn index(&self, index: usize) -> &[Cell; 9] {
        &self.0[index]
    }
}

impl<'a> IntoIterator for &'a Matrix {
    type Item = &'a [Cell; 9];
    type IntoIter = slice::Iter<'a, [Cell; 9]>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// convenience method 
impl Matrix {
    fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for (row_index, row) in self.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                cell.fmt(f)?; // content of cell
                
                if col_index == 2 || col_index == 5 {
                    write!(f, "|")?;
                }
            }
            
            writeln!(f)?; // new line
            
            if row_index == 2 || row_index == 5 {
                writeln!(f, "-----------------------")?;
            }
            
        }
        Ok(())
    }
}
fn main() {
    let matrix = Matrix::new();
    println!("{}", matrix);
}
