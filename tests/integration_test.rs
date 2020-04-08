mod common;

#[test]
fn sudoku_1() {
    let (matrix, solution) = common::read_matrix_and_solution(1);
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_2() {
    let (matrix, solution) = common::read_matrix_and_solution(2);
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_3() {
    let (matrix, solution) = common::read_matrix_and_solution(3);
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_4() {
    let (matrix, solution) = common::read_matrix_and_solution(4);
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_5() {
    let (matrix, solution) = common::read_matrix_and_solution(5);
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_6() {
    let (matrix, solution) = common::read_matrix_and_solution(6);
    assert_eq!(solution, matrix)
}
