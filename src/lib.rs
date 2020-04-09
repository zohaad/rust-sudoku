use std::ops::{ Index, IndexMut };
use std::fmt::{ self, Display, Formatter };
use std::slice;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Candidates([bool; 9]),
    Solution(usize),
    NoSolution,
}


impl Cell {
    // check if enum variant needs to be changed
    pub fn transform(self) -> Self {
        match self {
            Self::Candidates(c) => {
                let mut count = 0;
                let mut solution = 0;
                
                // count candidates
                for (idx, &candidate) in c.iter().enumerate() {
                    if candidate {
                        count += 1;
                        solution = idx + 1;
                    }
                }
                if count == 1 {
                    return Self::Solution(solution);
                } else if count == 0 {
                    return Self::NoSolution;
                }
                self
            },
            _ => self,
        }
    }
}

#[derive(PartialEq)]
pub struct Sudoku {
    matrix: [[Cell; 9]; 9],
    pub hints: VecDeque<[usize; 3]>,
}

impl Sudoku {
    pub fn new(contents: Vec<[usize; 3]>) -> Self {

        let mut sudoku = Self {
            // cell is an enum
            matrix: [[Cell::Candidates([true; 9]);9];9],
            hints: VecDeque::new(),
        };
        
        for &[row, col, value] in contents.iter() {
            sudoku[row][col] = Cell::Solution(value);
        }

        sudoku
    }

    pub fn solve(&mut self) {
        // find all hints 
        self.add_hints_to_queue();

        while let Some(hint) = self.hints.pop_front() {
            self.remove_hint_from_matrix(hint);
        } 
    }

    fn remove_hint_from_cell(&mut self, [i, j, solution]: [usize; 3]) {

        if let Cell::Candidates(c) = &mut self.matrix[i][j] { 
        // same as 
        // if let &mut Cell::Candidates(ref mut c) = &mut self.matrix[i][j] {
            c[solution - 1] = false;
            // change into new variant, if needed
            self.matrix[i][j] = self.matrix[i][j].transform();
            // check if we need to add new hints
            if let Cell::Solution(s) = self.matrix[i][j] {
                self.hints.push_back([i, j, s]);
            }
        }
    }

    fn remove_hint_from_matrix(&mut self, [row, col, solution]: [usize; 3]) {
        // remove from row
        for j in 0..9 {
            if j != col {
                self.remove_hint_from_cell([row, j, solution]);
            }
        }
        // remove from col
        for i in 0..9 {
            if i != row {
                self.remove_hint_from_cell([i, col, solution]);
            }
        }
        // remove from block
        let begin_row = (row / 3) * 3;
        let begin_col = (col / 3) * 3;
        for i in begin_row..begin_row+3 {
            for j in begin_col..begin_col+3 {
                if i != row && j != col {
                    self.remove_hint_from_cell([i, j, solution]);
                }
            }
        }
    }

    fn add_hints_to_queue(&mut self) {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if let Cell::Solution(s) = cell {
                    self.hints.push_back([i, j, s]);
                }
            }
        }
    }
    
    // TODO
    // pub fn rows(&self) {

    // }

    // pub fn cols(&self) {

    // }

    // pub fn blocks(&self) {

    // }
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


impl Index<usize> for Sudoku {
    // Idx: ?Sized (Sized trait bound removed) 
    type Output = [Cell; 9];

    fn index(&self, index: usize) -> &[Cell; 9] {
        &self.matrix[index]
    }
}

impl IndexMut<usize> for Sudoku {
    fn index_mut(&mut self, index: usize) -> &mut [Cell; 9] {
        &mut self.matrix[index]
    }
}

// correct definition
// trait IntoIterator {
//     type Item; // associated type of IntoIterator
//     type IntoIter: Iterator<Item = Self::Item>;
  
//     fn into_iter(self) -> Self::IntoIter;
//   }


impl<'a> IntoIterator for &'a Sudoku {
    type Item = &'a [Cell; 9];
    type IntoIter = slice::Iter::<'a, [Cell; 9]>;

    fn into_iter(self) -> Self::IntoIter {
        self.matrix.iter()
    }
}

// convenience method 
impl Sudoku {
    // some special syntax to get the associated types of implemented traits
    fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl Display for Sudoku {
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

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "\n{}", &self.to_string())
    }
}
