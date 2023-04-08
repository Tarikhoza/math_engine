use math_engine::parser::Parsable;
use math_engine::variable::Variable;

//Basic Operations

//Add
#[test]
fn add_two() {
    let x = Variable::from_tex("2").unwrap();
    let y = Variable::from_tex("2").unwrap();
    assert_eq!("4", (x + y).to_tex());
}

#[test]
fn add_two_same_suffix() {
    let x = Variable::from_tex("2x").unwrap();
    let y = Variable::from_tex("2x").unwrap();

    assert_eq!("4x", (x + y).to_tex());
}

#[test]
fn add_two_different_suffix() {
    let x = Variable::from_tex("2x").unwrap();
    let y = Variable::from_tex("2y").unwrap();
    assert_eq!("2x+2y", (x + y).to_tex());
}

#[test]
fn add_two_same_exponents() {
    let x = Variable::from_tex("2x^{2}").unwrap();
    let y = Variable::from_tex("2x^{2}").unwrap();

    assert_eq!("4x^{2}", (x + y).to_tex());
}

#[test]
fn add_two_different_exponents() {
    let x = Variable::from_tex("2x^{2}").unwrap();
    let y = Variable::from_tex("2y^{3}").unwrap();
    assert_eq!("2x^{2}+2y^{3}", (x + y).to_tex());
}

#[test]
fn sub_two() {
    let x = Variable::from_tex("6").unwrap();
    let y = Variable::from_tex("2").unwrap();
    assert_eq!("4", (x - y).to_tex());
}

#[test]
fn sub_two_same_suffix() {
    let x = Variable::from_tex("6x").unwrap();
    let y = Variable::from_tex("2x").unwrap();

    assert_eq!("4x", (x - y).to_tex());
}

#[test]
fn sub_two_different_suffix() {
    let x = Variable::from_tex("2x").unwrap();
    let y = Variable::from_tex("2y").unwrap();
    assert_eq!("2x-2y", (x - y).to_tex());
}

#[test]
fn sub_two_same_exponents() {
    let x = Variable::from_tex("6x^{2}").unwrap();
    let y = Variable::from_tex("2x^{2}").unwrap();

    assert_eq!("4x^{2}", (x - y).to_tex());
}

#[test]
fn sub_two_different_exponents() {
    let x = Variable::from_tex("2x^{2}").unwrap();
    let y = Variable::from_tex("2y^{3}").unwrap();
    assert_eq!("2x^{2}-2y^{3}", (x - y).to_tex());
}

#[test]
fn mul_two() {
    let x = Variable::from_tex("6").unwrap();
    let y = Variable::from_tex("2").unwrap();
    assert_eq!("12", (x * y).to_tex());
}

#[test]
fn mul_two_same_suffix() {
    let x = Variable::from_tex("6x").unwrap();
    let y = Variable::from_tex("2x").unwrap();

    assert_eq!("12x^{2}", (x * y).to_tex());
}

#[test]
fn mul_two_different_suffix() {
    let x = Variable::from_tex("2x").unwrap();
    let y = Variable::from_tex("2y").unwrap();
    assert_eq!("2x2y", (x * y).to_tex());
}

#[test]
fn mul_two_same_exponents() {
    let x = Variable::from_tex("6x^{2}").unwrap();
    let y = Variable::from_tex("2x^{2}").unwrap();

    assert_eq!("12x^{4}", (x * y).to_tex());
}

#[test]
fn mul_two_different_exponents() {
    let x = Variable::from_tex("2x^{2}").unwrap();
    let y = Variable::from_tex("2y^{3}").unwrap();
    assert_eq!("2x^{2}2y^{3}", (x * y).to_tex());
}
