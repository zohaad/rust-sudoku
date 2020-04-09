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
            matrix: [[Cell::Candidates([true; 9]); 9]; 9],
            hints: VecDeque::new(),
        };
        
        for &[row, col, value] in contents.iter() {
            sudoku[row][col] = Cell::Solution(value);
        }

        sudoku
    }

    fn invalid(&self) -> bool {
        for row in self.rows() {
            for col in row {
                if let Cell::NoSolution = col {
                    return true;
                }
            }
        }
        for x in 1usize..10 {
            for row in self.rows() {
                let mut count = 0;
                for cell in row {
                    if let Cell::Solution(s) = cell {
                        if x == *s {
                           count += 1; 
                        }
                    }
                }
                if count > 1 {
                    return true;
                }
            }
        }
        // if self.solved() {
        //     for x in 1usize..10 {
        //         for row in self.rows() {
        //             // count number of x's
        //             let mut count = 0;
        //             for cell in row {
        //                 if let Cell::Solution(s) = cell {
        //                     if &x == s {
        //                         count += 1;
        //                     }
        //                 }
        //             }
        //             if count != 1 {
        //                 return true;
        //             }
        //         }
        //         for col in self.cols() {
        //             // count number of x's
        //             let mut count = 0;
        //             for cell in col {
        //                 if let Cell::Solution(s) = cell {
        //                     if &x == s {
        //                         count += 1;
        //                     }
        //                 }
        //             }
        //             if count != 1 {
        //                 return true;
        //             }
        //         }
        //         for block in self.blocks() {
        //             // count number of x's
        //             let mut count = 0;
        //             for cell in block {
        //                 if let Cell::Solution(s) = cell {
        //                     if &x == s {
        //                         count += 1;
        //                     }
        //                 }
        //             }
        //             if count != 1 {
        //                 return true;
        //             }
        //         }
        //     }
        // }
        false
    }

    fn clone(&self) -> Sudoku {
        let mut sudoku = Self {
            matrix: [[Cell::Candidates([true; 9]); 9]; 9],
            hints: VecDeque::new(),
        };

        for (i, row) in self.rows().enumerate() {
            for (j, cell) in row.enumerate() {
                match cell {
                    Cell::NoSolution => sudoku.matrix[i][j] = Cell::NoSolution,
                    Cell::Solution(x) => sudoku.matrix[i][j] = Cell::Solution(x.clone()),
                    Cell::Candidates(c) => sudoku.matrix[i][j] = Cell::Candidates(c.clone()),
                }
            }
        }
        sudoku
    }
    fn solved(&self) -> bool {
        for row in self.rows() {
            for cell in row {
                match cell {
                    Cell::Solution(_) => (),
                    _ => return false,
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> Option<Self> {
        // logic rules 
        self.add_hints_to_queue();
        while let Some(hint) = self.hints.pop_front() {
            self.remove_hint_from_matrix(hint);
        }
        // generalization
        self.add_hints_generalization();
        while let Some(hint) = self.hints.pop_front() {
            self.remove_hint_from_matrix(hint);
        }
        if self.invalid() {
            return None;
        }
        if self.solved() {
            return Some(self.clone());
        }

        // enumeration

        for (i, row) in self.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Candidates(c) => {
                        let mut candidates = Vec::new();
                        let mut counter = 0;
                        for &x in c.iter() {
                            counter += 1;
                            if x {
                                candidates.push(counter);
                            }
                        }
                        for c in candidates {
                            let mut new_sudoku = self.clone();
                            new_sudoku.matrix[i][j] = Cell::Solution(c);
                            
                            match new_sudoku.solve() {
                                Some(solution) => return Some(solution),
                                None => continue,
                            }
                        }
                    },
                    _ => continue,
                }
            }
        }
    
        None
    }

    pub fn add_hints_generalization(&mut self) {
        // every number can be only once in every row, col and block
        // for x in 0..9 {

        // }
        let mut solved = Vec::new();
        for x in 0..9 {
            for (i, row) in self.rows().enumerate() {
                let mut count = 0;
                let mut j_saved = 0;
                for (j, cell) in row.enumerate() {
                    if let Cell::Candidates(s) = cell {
                        if s[x] {
                            count += 1;
                            j_saved = j;
                        }
                    }
                }
                if count == 1 { // only one value of x 
                    solved.push([i, j_saved, x + 1]);
                }
            }
            for (j, col) in self.cols().enumerate() {
                let mut count = 0;
                let mut i_saved = 0;
                for (i, cell) in col.enumerate() {
                    if let Cell::Candidates(s) = cell {
                        if s[x] {
                            count += 1;
                            i_saved = i;
                        }
                    }
                }
                if count == 1 {
                    solved.push([i_saved, j, x + 1]);
                }
            }
            for (b, block) in self.blocks().enumerate() {
                let mut count = 0;
                let mut k_saved = 0;
                for (k, cell) in block.enumerate() {
                    if let Cell::Candidates(s) = cell {
                        if s[x] {
                            count += 1;
                            k_saved = k;
                        }
                    }

                    
                }
                if count == 1 {
                    // logic for i and j retrieval
                    let i = (b / 3) * 3 + k_saved / 3;
                    let j = (b % 3) * 3 + k_saved % 3;
                    solved.push([i, j, x + 1]);
                }   
            }
          
            
        }
        for [i, j, x] in solved {
            self.matrix[i][j] = Cell::Solution(x);
            self.hints.push_back([i, j, x]);
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

    fn rows(&self) -> RowIter {
        RowIter { row: 0, matrix: &self.matrix }
    }

    fn cols(&self) -> ColIter {
        ColIter { col: 0, matrix: &self.matrix }
    }

    fn blocks(&self) -> BlockIter {
        BlockIter { block: 0, matrix: &self.matrix }
    }
}

#[derive(Copy, Clone)]
struct Row<'a> {
    row: usize,
    col: usize,
    matrix: &'a [[Cell; 9]; 9],
}

impl<'a> Iterator for Row<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.matrix.get(self.row)
            .and_then(|row| row.get(self.col))
            .map(|cell| {
                self.col += 1;
                cell
            })
    }
}

struct RowIter<'a> {
    row: usize,
    matrix: &'a [[Cell; 9]; 9],
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < 9 {
            let row = Row {
                row: self.row,
                col: 0,
                matrix: self.matrix,
            };
            self.row += 1;
            Some(row)
        } else {
            None
        }
    }
}


#[derive(Copy, Clone)]
struct Column<'a> {
    row: usize,
    col: usize,
    matrix: &'a [[Cell; 9]; 9],
}

impl<'a> Iterator for Column<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.matrix.get(self.row)
            .and_then(|row| row.get(self.col))
            .map(|cell| {
                self.row += 1;
                cell
            })
    }
}

struct ColIter<'a> {
    col: usize,
    matrix: &'a [[Cell; 9]; 9],
}

impl<'a> Iterator for ColIter<'a> {
    type Item = Column<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < 9 {
            let column = Column {
                row: 0,
                col: self.col,
                matrix: self.matrix,
            };
            self.col += 1;
            Some(column)
        } else {
            None
        }
    }
}


#[derive(Copy, Clone)]
struct Block<'a> {
    block: usize,
    count: usize,
    matrix: &'a [[Cell; 9]; 9],
}

impl<'a> Iterator for Block<'a> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 9 {
            // offset within block + top-left cell
            let row = self.count / 3 + (self.block / 3) * 3;
            let col = self.count % 3 + (self.block % 3) * 3;
            self.count += 1;
            Some(&self.matrix[row][col])
        } else {
            None
        }
    }
}

struct BlockIter<'a> {
    block: usize,
    matrix: &'a [[Cell; 9]; 9],
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = Block<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.block < 9 {
            let block = Block {
                block: self.block,
                count: 0,
                matrix: self.matrix,
            };
            self.block += 1;
            Some(block)
        } else {
            None
        }
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