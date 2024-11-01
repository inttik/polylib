use polylib::polynom::Polynomial;
use polylib::polynom::{X, Y};
use polylib::{One, Zero};

mod common;
use common::same_coef;
use common::substitude_check;

type TestType = i32;
type Poly = Polynomial<TestType, X<TestType>>;

#[test]
fn test_const() {
    let c = Poly::new_const(0);
    same_coef(c, vec![0]);

    let c = Poly::new_const(1);
    same_coef(c, vec![1]);

    let c = Poly::new_const(-1);
    same_coef(c, vec![-1]);
}

#[test]
fn test_from_coefs() {
    let c = Poly::from_coefs(vec![0, 0, 0]);
    assert_eq!(c.len(), 0);

    let c = Poly::from_coefs(vec![3, 0, 1, 2]);
    assert_eq!(c.get(0).expect("coef 0 should be 3").clone(), 3);
    assert_eq!(c.get(2).expect("coef 2 should be 1").clone(), 1);
    assert_eq!(c.get(3).expect("coef 3 should be 2").clone(), 2);

    assert!(c.get(1).is_none(), "coef 1 should be None");
    assert!(c.get(4).is_none(), "coef 4 should be None");
}

#[test]
fn test_const_add() {
    let c = Poly::new_const(0);
    let c = c + 32;
    same_coef(c, vec![32]);

    let c = Poly::from_coefs(vec![0, 2, 3]);
    let c = c + 32;
    same_coef(c, vec![32, 2, 3]);

    let c = Poly::from_coefs(vec![1, 2]);
    let c = c + 32;
    same_coef(c, vec![33, 2]);

    let c = Poly::from_coefs(vec![1, 2]);
    let c = c + -32;
    same_coef(c, vec![-31, 2]);
}

#[test]
fn test_add() {
    let a = Poly::from_coefs(vec![1, 1]);
    let b = Poly::from_coefs(vec![2, 3]);
    same_coef(a + b, vec![3, 4]);

    let a = Poly::from_coefs(vec![0, 1, 1]);
    let b = Poly::from_coefs(vec![2, 3]);
    same_coef(a + b, vec![2, 4, 1]);

    let a = Poly::from_coefs(vec![1]);
    let b = Poly::from_coefs(vec![0, 1]);
    same_coef(a + b, vec![1, 1]);
}

#[test]
fn test_zero() {
    let zero = Poly::zero();
    same_coef(zero, vec![]);

    let a = Poly::from_coefs(vec![1, 0, 1]);
    same_coef(a.clone() + Poly::zero(), vec![1, 0, 1]);
    same_coef(Poly::zero() + a, vec![1, 0, 1]);

    let a = Poly::from_coefs(vec![0, 2]);
    same_coef(a.clone() + Poly::zero(), vec![0, 2]);
    same_coef(Poly::zero() + a, vec![0, 2]);

    same_coef(Poly::zero() + Poly::zero(), vec![0]);
}

#[test]
fn test_const_sub() {
    let c = Poly::from_coefs(vec![5, 5, 5]);
    same_coef(c - 3, vec![2, 5, 5]);

    let c = Poly::from_coefs(vec![5, 5, 5]);
    same_coef(c - 7, vec![-2, 5, 5]);

    let c = Poly::from_coefs(vec![0, 1]);
    same_coef(c - 1, vec![-1, 1]);
}

#[test]
fn test_sub() {
    let a = Poly::from_coefs(vec![5, 5, 5]);
    let b = Poly::from_coefs(vec![2, 0, 7]);
    same_coef(a - b, vec![3, 5, -2]);

    let a = Poly::from_coefs(vec![1, 0, 0, 1]);
    let b = Poly::from_coefs(vec![1, 7]);
    same_coef(a - b, vec![0, -7, 0, 1]);

    let a = Poly::from_coefs(vec![1, 7]);
    let b = Poly::from_coefs(vec![1, 0, 0, 1]);
    same_coef(a - b, vec![0, 7, 0, -1]);
}

#[test]
fn test_neg() {
    let a = Poly::from_coefs(vec![1, 0, -1]);
    same_coef(-a, vec![-1, 0, 1]);

    let a = Poly::from_coefs(vec![0]);
    same_coef(-a, vec![0]);
}

#[test]
fn test_const_mul() {
    let a = Poly::from_coefs(vec![2, 1, 0, -2]);
    same_coef(a.clone() * 3, vec![6, 3, 0, -6]);
    same_coef(a.clone() * 1, vec![2, 1, 0, -2]);
    same_coef(a.clone() * -1, vec![-2, -1, 0, 2]);
    same_coef(a * 0, vec![0]);

    let a = Poly::from_coefs(vec![0]);
    same_coef(a.clone() * 3, vec![0]);
    same_coef(a.clone() * 1, vec![0]);
    same_coef(a.clone() * -1, vec![0]);
    same_coef(a * 0, vec![0]);
}

#[test]
fn test_mul() {
    let a = Poly::from_coefs(vec![2, 1, 0, -2]);
    let b = Poly::from_coefs(vec![3]);
    same_coef(a * b, vec![6, 3, 0, -6]);

    let a = Poly::from_coefs(vec![1, 1]);
    let b = Poly::from_coefs(vec![-1, 1]);
    same_coef(a * b, vec![-1, 0, 1]);

    let a = Poly::from_coefs(vec![1, 0, 0, 2, 0, 0]);
    let b = Poly::from_coefs(vec![2, 0, -1]);
    same_coef(a * b, vec![2, 0, -1, 4, 0, -2]);
}

#[test]
fn test_one() {
    let one = Poly::one();
    same_coef(one, vec![1]);

    let a = Poly::from_coefs(vec![3, -2, 0, 1]);
    same_coef(a.clone() * Poly::one(), vec![3, -2, 0, 1]);
    same_coef(Poly::one() * a.clone(), vec![3, -2, 0, 1]);

    let a = Poly::from_coefs(vec![0]);
    same_coef(a.clone() * Poly::one(), vec![0]);
    same_coef(Poly::one() * a.clone(), vec![0]);

    same_coef(Poly::one() * Poly::one(), vec![1]);
}

#[test]
fn test_pow() {
    let a = Poly::from_coefs(vec![2]);
    same_coef(a.pow(10), vec![1024]);

    let a = Poly::from_coefs(vec![0, -1]);
    same_coef(a.pow(3), vec![0, 0, 0, -1]);

    let a = Poly::from_coefs(vec![-1, 1]);
    same_coef(a.clone().pow(3), vec![-1, 3, -3, 1]);
    same_coef(a.clone().pow(1), vec![-1, 1]);
    same_coef(a.pow(0), vec![1]);
}

#[test]
fn substitude_i32() {
    let simple = Poly::from_coefs(vec![1]);
    substitude_check(simple, vec![-2, -1, 0, 1, 2], vec![1, 1, 1, 1, 1]);

    let a = Poly::from_coefs(vec![0, 1, 1]);
    substitude_check(a, vec![-2, -1, 0, 1, 2], vec![2, 0, 0, 2, 6]);

    let a = Poly::from_coefs(vec![1, 0, 1]);
    substitude_check(a, vec![-2, -1, 0, 1, 2], vec![5, 2, 1, 2, 5]);
}

#[test]
fn substitude_poly() {
    type PolyX = Polynomial<i32, X<i32>>;
    type PolyY = Polynomial<i32, Y<i32>>;

    let x_poly = PolyX::from_coefs(vec![0, 1, 1]);
    let y_poly = PolyY::from_coefs(vec![1, 0, 1]);

    // (1 + y^2) + (1 + y^2)^2
    let xy = x_poly.clone().rsubstitude(y_poly.clone());
    same_coef(xy, vec![2, 0, 3, 0, 1]);

    // 1 + (x + x^2)^2
    let yx = y_poly.rsubstitude(x_poly);
    same_coef(yx, vec![1, 0, 1, 2, 1]);
}

#[test]
fn test_gc() {
    let x = X::<i32>::default();
    let c = Poly::from_coefs(vec![1, 1]);
    let c = c + x.pow(1);
    assert_eq!(
        c.get(1).expect("has coef x").clone(),
        1,
        "perhaps, gc was called"
    );
    assert_eq!(c.substitude(1), 3, "(1 + x + x) where x = 1 is 3");
    let c = c.reduce();
    assert_eq!(
        c.get(1).expect("has coef x").clone(),
        2,
        "(1 + x + x) should transform into (1 + 2x)"
    );
}

#[test]
fn test_build() {
    let x = X::<i32>::default();
    let poly = Polynomial::default() + 1 + x.pow(2) * 3 - x.pow(1) + 8;
    same_coef(poly, vec![9, -1, 3]);

    let poly = Polynomial::default() + 1 + (x ^ 2) * 3 - (x ^ 1) + 8;
    same_coef(poly, vec![9, -1, 3]);
}
