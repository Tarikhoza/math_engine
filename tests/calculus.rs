use math_engine::math::simplifiable::Simplifiable;
use math_engine::parser::{Parsable, ParsableGenerics};

macro_rules! parser_eq {
    ($input:expr, $expected:expr) => {
        assert_eq!(
            //TODO remove double simplify
            $input.parse_math().unwrap().simplify().simplify().to_tex(),
            $expected
        );
    };
}

#[test]
fn sum() {
    //   parser_eq!("\\sum_{n=1}^{5}n^{2}", "55");
    //   parser_eq!("\\sum_{n=1}^{5}3n+2", "55");
    parser_eq!("\\sum_{n=3}^{6}2^{n}", "126");
}
