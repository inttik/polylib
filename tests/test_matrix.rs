use polylib::custom_types::Matrix;
use polylib::polynom::Polynomial;
use polylib::polynom::X;

mod common;
use common::substitude_check;

type Poly = Polynomial<i32, X<i32>>;


#[test]
fn test_diagonal_matrix() {
    type Type = Matrix<3, 3, i32>;
    let x = X::<i32>::default();
    let poly = (x ^ 3) + (x ^ 1) + 1;

    let input = vec![
        Type::eye(-2),
        Type::eye(-1),
        Type::eye(0),
        Type::eye(1),
        Type::eye(2),
    ];

    let expect = vec! [
        Type::eye(-9),
        Type::eye(-1),
        Type::eye(1),
        Type::eye(3),
        Type::eye(11),
    ];

    substitude_check(poly, input, expect);
}

#[test]
fn test_test_funny_matrix() {
    type Type = Matrix<2, 2, i32>;
    let poly = Poly::from_coefs(vec![2, 3, 4, 5, 6, 7]);

    let input = vec![
        Type::from_data(vec![0, 1, 1, 0]),
    ];

    let expect = vec![
        Type::from_data(vec![12, 15, 15, 12]),
    ];

    substitude_check(poly, input, expect);
}

#[test]
fn test_nilpotent_matrix() {
    type Type = Matrix<3, 3, i32>;
    let x = X::<i32>::default();
    let poly = (x ^ 1000) + (x ^ 1);

    let input = vec![
        Type::from_data(vec![0, 0, 0, 0, 0, 0, 1, 1, 0]),
    ];

    let expect = vec![
        Type::from_data(vec![0, 0, 0, 0, 0, 0, 1, 1, 0]),
    ];

    substitude_check(poly, input, expect);
}
