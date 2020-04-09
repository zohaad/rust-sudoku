mod common;

fn sudoku(n: usize) {
    let (mut matrix, solution) = common::read_matrix_and_solution(n);
    matrix.solve();
    assert_eq!(solution, matrix);
}

#[test]
fn sudoku_1() {
    sudoku(1);
}

#[test]
fn sudoku_2() {
    sudoku(2);
}

#[test]
fn sudoku_3() {
    sudoku(3)
}

#[test]
fn sudoku_4() {
    sudoku(4);
}

#[test]
fn sudoku_5() {
    sudoku(5);
}

#[test]
fn sudoku_6() {
    sudoku(6);
}
