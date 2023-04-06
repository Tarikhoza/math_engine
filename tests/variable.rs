use math_engine::parser::Parsable;
use math_engine::variable::Variable;

//Basic Operations

//Add
#[test]
fn add_tw() {
    let x = Variable::new("2");
    let y = Variable::new("2");
    assert_eq!("4", (x + y).to_tex());
}

#[test]
fn add_two_same_suffix() {
    let x = Variable::new("2x");
    let y = Variable::new("2x");

    assert_eq!("4x", (x + y).to_tex());
}

#[test]
fn add_two_different_suffix() {
    let x = Variable::new("2x");
    let y = Variable::new("2y");
    assert_eq!("2x+2y", (x + y).to_tex());
}

#[test]
fn add_two_same_exponents() {
    let x = Variable::new("2x^{2}");
    let y = Variable::new("2x^{2}");

    assert_eq!("4x^{2}", (x + y).to_tex());
}

#[test]
fn add_two_different_exponents() {
    let x = Variable::new("2x^{2}");
    let y = Variable::new("2y^{3}");
    assert_eq!("2x^{2}+2y^{3}", (x + y).to_tex());
}

#[test]
fn sub_two() {
    let x = Variable::new("6");
    let y = Variable::new("2");
    assert_eq!("4", (x - y).to_tex());
}

#[test]
fn sub_two_same_suffix() {
    let x = Variable::new("6x");
    let y = Variable::new("2x");

    assert_eq!("4x", (x - y).to_tex());
}

#[test]
fn sub_two_different_suffix() {
    let x = Variable::new("2x");
    let y = Variable::new("2y");
    assert_eq!("2x-2y", (x - y).to_tex());
}

#[test]
fn sub_two_same_exponents() {
    let x = Variable::new("6x^{2}");
    let y = Variable::new("2x^{2}");

    assert_eq!("4x^{2}", (x - y).to_tex());
}

#[test]
fn sub_two_different_exponents() {
    let x = Variable::new("2x^{2}");
    let y = Variable::new("2y^{3}");
    assert_eq!("2x^{2}-2y^{3}", (x - y).to_tex());
}

#[test]
fn mul_two() {
    let x = Variable::new("6");
    let y = Variable::new("2");
    assert_eq!("12", (x * y).to_tex());
}

#[test]
fn mul_two_same_suffix() {
    let x = Variable::new("6x");
    let y = Variable::new("2x");

    assert_eq!("12x^{2}", (x * y).to_tex());
}

#[test]
fn mul_two_different_suffix() {
    let x = Variable::new("2x");
    let y = Variable::new("2y");
    assert_eq!("2x2y", (x * y).to_tex());
}

#[test]
fn mul_two_same_exponents() {
    let x = Variable::new("6x^{2}");
    let y = Variable::new("2x^{2}");

    assert_eq!("12x^{4}", (x * y).to_tex());
}

#[test]
fn mul_two_different_exponents() {
    let x = Variable::new("2x^{2}");
    let y = Variable::new("2y^{3}");
    assert_eq!("2x^{2}2y^{3}", (x * y).to_tex());
}
