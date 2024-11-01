use polylib::custom_types::Zn;
use polylib::polynom::Polynomial;
use polylib::polynom::X;

mod common;
use common::same_coef;
use common::substitude_check;

type TestType = Zn<3>;
type Poly = Polynomial<TestType, X<TestType>>;

#[test]
fn test_zn() {
    let coef = vec![
        TestType::new(1),
        TestType::new(4),
        TestType::new(6),
        TestType::new(7),
    ];
    let poly = Poly::from_coefs(coef.clone());
    same_coef(poly.clone(), coef);
    substitude_check(
        poly,
        vec![TestType::new(0), TestType::new(1), TestType::new(2)],
        vec![TestType::new(1), TestType::new(0), TestType::new(2)],
    );
}
