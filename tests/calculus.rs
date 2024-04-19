use math_engine::lexer::Tokenisable;
use math_engine::math::simplifiable::Simplifiable;
use math_engine::parser::{Parsable, Parser};

macro_rules! parser_eq {
    ($input:expr, $expected:expr) => {
        assert_eq!(
            //TODO remove double simplify
            Parser::new($input.tokenise().unwrap())
                .parse()
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
    //    parser_eq!("\\sum_{n=1}^{4}(\\frac{3}{2})^{n-1}", "\\frac{65}{8}");
    parser_eq!("\\sum_{n=1}^{4}8", "32");
    parser_eq!("\\sum_{n=1}^{8}7", "56");
    parser_eq!("\\sum_{n=1}^{5}9", "45");
    parser_eq!("\\sum_{i=1}^{5}i", "15");
    parser_eq!("\\sum_{i=1}^{4}6i", "60");
    parser_eq!("\\sum_{i=1}^{5}7i-3", "90");
    //parser_eq!("\\sum_{i=1}^{5}(n+n^{2}+n^{3})", "97");
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

#[test]
fn factorial() {
    parser_eq!("6!", "720");
    parser_eq!("5!", "120");
    parser_eq!("4!", "24");
    parser_eq!("3!", "6");
    parser_eq!("2!", "2");
    parser_eq!("1!", "1");
    parser_eq!("\\frac{5!}{2!2!}", "30");
    parser_eq!("\\frac{5!}{4!}", "5");
    parser_eq!("\\frac{6!}{2!}", "360");
    parser_eq!("\\frac{10!}{3!2!4!}", "6300");

    parser_eq!("\\frac{7!}{5!}", "120");
    parser_eq!("\\frac{3!}{4!}*0!", "\\frac{1}{4}");
    parser_eq!("\\frac{4!b!}{b!}!", "24");
    parser_eq!("4!*3!*2!", "288");
}

#[test]
fn binomial() {
    parser_eq!("\\binom{2}{1}", "2");
    parser_eq!("\\binom{8}{4}", "70");
    parser_eq!("\\binom{5}{3}*\\binom{289}{0}", "10");
    parser_eq!("\\binom{7}{2}", "21");
    parser_eq!("\\binom{3}{1}", "3");
    parser_eq!("\\binom{4}{2}*\\binom{3}{2}", "18");
}

//   #[test]
//   fn sum_converge() {
//       // parser_eq!("\\sum_{n=1}^{\\inf}8(\\frac{2}{3})^{n-1}", "24");
//   }
