use math_engine::math::simplifiable::Simplifiable;
use math_engine::parser::{Parsable, ParsablePrimitive};

macro_rules! parser_eq {
    ($input:expr, $expected:expr) => {
        assert_eq!(
            //TODO remove double simplify
            $input
                .parse_math()
                .unwrap()
                .simplify()
                .simplify()
                .simplify()
                .to_tex(),
            $expected
        );
    };
}

#[test]
fn sum() {
    parser_eq!("\\sum_{n=1}^{5}n^{2}", "55");
    parser_eq!("\\sum_{n=1}^{5}3n+2", "55");
    parser_eq!("\\sum_{n=3}^{6}2^{n}", "120");
    parser_eq!("\\sum_{n=1}^{100}4n+5", "20700");
    parser_eq!("\\sum_{n=1}^{4}(\\frac{3}{2})^{n-1}", "\\frac{65}{8}");
    parser_eq!("\\sum_{n=1}^{4}8", "32");
    parser_eq!("\\sum_{n=1}^{8}7", "56");
    parser_eq!("\\sum_{n=1}^{5}9", "45");
    parser_eq!("\\sum_{i=1}^{5}i", "15");
    parser_eq!("\\sum_{i=1}^{4}6i", "60");
    parser_eq!("\\sum_{i=1}^{5}7i-3", "90");
}

#[test]
fn product() {
    parser_eq!("\\prod_{n=1}^{5}n^{2}", "14400");
    parser_eq!("\\prod_{n=1}^{5}3n+2", "104720");
    parser_eq!("\\prod_{n=3}^{6}2^{n}", "262144");
    parser_eq!("\\prod_{n=1}^{4}(\\frac{3}{2})^{n-1}", "\\frac{729}{64}");
    parser_eq!("\\prod_{n=1}^{4}8", "4096");
    parser_eq!("\\prod_{n=1}^{8}7", "5764801");
    parser_eq!("\\prod_{n=1}^{5}9", "59049");
    parser_eq!("\\prod_{i=1}^{5}i", "120");
    parser_eq!("\\prod_{i=1}^{4}6i", "31104");
    parser_eq!("\\prod_{i=1}^{5}7i-3", "633600");
}

//   #[test]
//   fn sum_converge() {
//       // parser_eq!("\\sum_{n=1}^{\\inf}8(\\frac{2}{3})^{n-1}", "24");
//   }
