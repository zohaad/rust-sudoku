use std::ops::{ Index, IndexMut };
use std::fmt::{ self, Display, Formatter };
use std::slice;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Cell {
    Candidates([bool; 9]),
    Solution(u8),
    NoSolution,
}

#[derive(PartialEq, Debug)]
pub struct Matrix([[Cell; 9]; 9]);

impl Matrix {
    pub fn new(contents: Vec<[u8; 3]>) -> Self {

        let mut matrix = Matrix(
            // cell is an enum
            [[Cell::Candidates([true; 9]);9];9]
            );
        
        for [row, col, value] in contents.iter() {
            matrix[*row as usize][*col as usize] = Cell::Solution(*value);
        }

        matrix
    }
}


impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // use write!(f, "{}", something)
        // which returns a Result
        match self {
            Cell::Candidates(_) => write!(f, "  "),
            Cell::Solution(x) => write!(f, "{} ", x),
            Cell::NoSolution => write!(f, "? "),
        }
    }
}


impl Index<usize> for Matrix {
    type Output = [Cell; 9];

    fn index(&self, index: usize) -> &[Cell; 9] {
        &self.0[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (row_index, row) in self.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                cell.fmt(f)?; // content of cell
                
                if col_index == 2 || col_index == 5 {
                    write!(f, "| ")?;
                }
            }
            
            if row_index < 8 {
                writeln!(f)?; // new line
            }
            
            if row_index == 2 || row_index == 5 {
                writeln!(f, "---------------------")?;
            }
        }
        Ok(())
    }
}
