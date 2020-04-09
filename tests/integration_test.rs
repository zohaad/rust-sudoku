mod common;

#[test]
fn sudoku_1() {
    let (mut matrix, solution) = common::read_matrix_and_solution(1);
    matrix.solve();
    assert_eq!(solution, matrix);
}

#[test]
fn sudoku_2() {
    let (mut matrix, solution) = common::read_matrix_and_solution(2);
    matrix.solve();
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_3() {
    let (mut matrix, solution) = common::read_matrix_and_solution(3);
    matrix.solve();
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_4() {
    let (mut matrix, solution) = common::read_matrix_and_solution(4);
    matrix.solve();
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_5() {
    let (mut matrix, solution) = common::read_matrix_and_solution(5);
    matrix.solve();
    assert_eq!(solution, matrix)
}

#[test]
fn sudoku_6() {
    let (mut matrix, solution) = common::read_matrix_and_solution(6);
    matrix.solve();
    assert_eq!(solution, matrix)
}
