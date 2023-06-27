use math_engine::parser::{Parsable, ParsableGenerics};

macro_rules! should_error {
    ($input:expr ) => {
        if let Ok(_math) = $input.parse_math() {
            println!("Error for: '{}' => '{}'", $input, _math.to_tex());
            panic!("Should have returned and error");
        }
    };
}

#[test]
fn dangling_op() {
    should_error!("-3-");
    should_error!("3x+");
    should_error!("-2*2-");
    should_error!("-2*2=");
    should_error!("-2*2<=");
    should_error!("=3x+3");
    should_error!("=3=");
}
