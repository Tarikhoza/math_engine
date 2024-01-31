fn all_combinations() {
    use math_engine::math::algebra::operations::Operations;
    use math_engine::math::simplifiable::Simplifiable;
    use math_engine::math::*;
    use math_engine::parser::*;

    let a = vec![
        "2".parse_math().unwrap(),
        "8x".parse_math().unwrap(),
        "\\frac{2}{3}".parse_math().unwrap(),
        "3x^{2}+3".parse_math().unwrap(),
        "(3+2x)".parse_math().unwrap(),
    ];
    let mut start_add = Vec::<String>::new();
    let mut start_sub = Vec::<String>::new();
    let mut start_mul = Vec::<String>::new();
    let mut start_div = Vec::<String>::new();

    let mut result_add = Vec::<Math>::new();
    let mut result_sub = Vec::<Math>::new();
    let mut result_mul = Vec::<Math>::new();
    let mut result_div = Vec::<Math>::new();

    for i in a.iter() {
        for j in a.iter() {
            start_add.push(format!("{}+{}", i.to_tex(), j.to_tex()));
            start_sub.push(format!("{}-{}", i.to_tex(), j.to_tex()));
            start_mul.push(format!("{}*{}", i.to_tex(), j.to_tex()));
            start_div.push(format!("{}/{}", i.to_tex(), j.to_tex()));
            result_add.push(i.add(&j).simplify());
            result_sub.push(i.sub(&j).simplify());
            result_mul.push(i.mul(&j).simplify());
            result_div.push(i.div(&j).simplify());
        }
    }
}
