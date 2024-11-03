use polylib::custom_types::Zn;
use polylib::custom_types::Matrix;
use polylib::polynom::X;

#[test]
fn test_calcs_are_fast() {
    type Inner = Zn<9_999>;
    type Type = Matrix<5, 5, Inner>;

    let x = X::<i32>::default();
    let poly = (x ^ 2_000_000_000) * 23 + (x ^ 1_321_654) * 5 + (x ^ 1337) * 7 + (x ^ 228);

    let mut input_data = Vec::<Inner>::new();

    for i in 1..=25 {
        input_data.push(Inner::new(i));
    }

    let input = Type::from_data(input_data);

    let _ = poly.substitude(input);
}
