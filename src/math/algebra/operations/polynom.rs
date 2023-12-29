use crate::castable::Castable;
use crate::math::algebra::exponentable::Exponentable;
use crate::math::algebra::operations::{
    Operations as AlgebraOperations, Operator as AlgebraOperator,
};
use crate::math::algebra::polynom::{Polynom, PolynomPart};
use crate::math::algebra::undefined::Undefined;
use crate::math::simplifiable::Simplifiable;
use crate::math::Math;
use crate::parser::Parsable;

use crate::logging::{env_error, env_info, env_warn};

impl Simplifiable for Polynom {
    fn simplify(&self) -> Math {
        env_info(
            "operations",
            format!("### simplify polynom before {} ###", self.to_tex()),
        );
        if let Some(PolynomPart::Math(m)) = self.parts.first() {
            if self.parts.len() == 1 {
                return m.as_math();
            }
        }
        let result = self
            .morph_ops()
            .simplify_par()
            .simplify_exp()
            .simplify_mul_div()
            .simplify_add_sub();

        env_info(
            "operations",
            format!("###simplify polynom after {}###\n", result.to_tex()),
        );
        if let Some(PolynomPart::Math(m)) = result.parts.first() {
            if result.parts.len() == 1 {
                return m.as_math();
            }
        }
        result.as_math()
    }
}
impl AlgebraOperations for Polynom {
    fn add_self(&self, other: &Polynom) -> Math {
        Math::Polynom(Polynom {
            parts: {
                let mut parts = self.parts.clone();
                parts.push(AlgebraOperator::Addition.as_polynom_part());
                parts.extend(other.parts.clone());
                parts
            },
        })
    }

    fn sub_self(&self, other: &Polynom) -> Math {
        Math::Polynom(Polynom {
            parts: {
                let mut parts = self.parts.clone();
                parts.push(AlgebraOperator::Subtraction.as_polynom_part());
                parts.extend(other.parts.clone());
                parts
            },
        })
    }

    fn mul_self(&self, other: &Polynom) -> Math {
        let mut parts: Vec<PolynomPart> = vec![];
        for i in self.to_vector().factors.iter() {
            for j in other.to_vector().factors.iter() {
                let product = i.mul(j);
                parts.push(product.as_polynom_part());
                parts.push(AlgebraOperator::Addition.as_polynom_part());
            }
        }
        parts.pop();

        Polynom { parts }.as_math()
    }

    fn div_self(&self, _other: &Polynom) -> Math {
        todo!("you have to implement division between polynomes")
    }

    fn add(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.add_self(p),
            Math::Variable(v) => self.add_self(&v.as_polynom()),
            Math::Braces(b) => self.add_self(&b.as_polynom()),
            Math::Fraction(f) => self.as_fraction().add_self(f),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!(),
        }
    }

    fn sub(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.sub_self(p),
            Math::Variable(v) => self.sub_self(&v.as_polynom()),
            Math::Braces(b) => self.sub_self(&b.as_polynom()),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            Math::Fraction(f) => self.as_fraction().sub_self(f),
            _ => todo!(),
        }
    }
    fn mul(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Polynom(p) => self.mul_self(p),
            Math::Variable(v) => self.mul_self(&v.as_polynom()),
            Math::Braces(b) => self.mul_self(&b.as_polynom()),
            Math::Fraction(f) => self.as_fraction().mul_self(f),
            Math::Undefined(_u) => Math::Undefined(Undefined {}),
            _ => todo!("did not implement mul with polynom"),
        }
    }
    fn div(&self, rhs: &Math) -> Math {
        match rhs {
            Math::Fraction(f) => self.as_fraction().div_self(f),
            _ => todo!("did not implement div with polynom"),
        }
    }

    fn substitute(&self, suffix: &str, _math: Math) -> Math {
        let mut new_poly = self.clone();
        for part in new_poly.parts.iter_mut() {
            if let PolynomPart::Math(math) = part {
                *part = math.substitute(suffix, _math.clone()).as_polynom_part();
            }
        }
        new_poly.as_math()
    }
    fn get_all_suffixes(&self) -> Vec<String> {
        let mut suf: Vec<String> = vec![];
        for i in self.parts.iter() {
            if let PolynomPart::Math(m) = i {
                suf.extend(m.get_all_suffixes())
            }
        }
        suf.sort();
        suf.dedup();
        suf
    }
}

//  Simplify helper functions
impl Polynom {
    //  PEMDAS
    //  P - Parentheses
    pub fn simplify_par(&self) -> Polynom {
        env_info(
            "operations",
            format!("simplify_par before {}", self.to_tex()),
        );

        let mut parts: Vec<PolynomPart> = self.parts.clone();
        for part in parts.iter_mut() {
            if let PolynomPart::Math(Math::Braces(b)) = part {
                *part = b.simplify().as_polynom_part();
            }
        }

        env_info(
            "operations",
            format!("simplify_par after {}", self.to_tex()),
        );
        Polynom { parts }
    }
    //  E - Exponents (Powers and Square Roots, etc.)

    pub fn simplify_exp(&self) -> Polynom {
        env_info(
            "operations",
            format!("simplify_exp before {}", self.to_tex()),
        );

        let mut parts: Vec<PolynomPart> = self.parts.clone();

        for part in parts.iter_mut() {
            match part {
                PolynomPart::Math(Math::Braces(b)) => {
                    env_info("operations", format!("applying exponent to {}", b.to_tex()));
                    *part = b.apply_exponent().as_polynom_part();
                    env_info("operations", format!("after exponentiation to {:#?}", part));
                }
                PolynomPart::Math(Math::Variable(v)) => {
                    env_info("operations", format!("applying exponent to {}", v.to_tex()));
                    *part = v.simplify().as_polynom_part();
                    env_info("operations", format!("after exponentiation to {:#?}", part));
                }
                _ => {}
            }
        }

        env_info(
            "operations",
            format!("simplify_exp after {}", self.to_tex()),
        );

        Polynom { parts }
    }

    //  MD - Multiplication and Division (left-to-right)
    pub fn simplify_mul_div(&self) -> Polynom {
        if !self
            .parts
            .contains(&PolynomPart::Operator(AlgebraOperator::Multiplication))
            && !self
                .parts
                .contains(&PolynomPart::Operator(AlgebraOperator::InvMulti))
            && !self
                .parts
                .contains(&PolynomPart::Operator(AlgebraOperator::Division))
        {
            env_info(
                "operations",
                format!(
                    "simplify_mul_div does not contain any mul or div ops {}",
                    self.to_tex()
                ),
            );
            return self.clone();
        }

        env_info(
            "operations",
            format!("simplify_mul_div before {}", self.to_tex()),
        );

        let mut parts = Vec::new();

        for (i, window) in self.parts.windows(3).enumerate() {
            if let (
                Some(PolynomPart::Math(lhs)),
                Some(PolynomPart::Operator(op)),
                Some(PolynomPart::Math(rhs)),
            ) = (window.get(0), window.get(1), window.get(2))
            {
                match *op {
                    AlgebraOperator::Multiplication | AlgebraOperator::InvMulti => {
                        let part = lhs.mul(rhs);
                        if part.to_tex() != "0" {
                            parts.push(part.as_polynom_part());
                        }
                        parts.extend_from_slice(self.parts.get(i + 3..).unwrap_or(&[]));
                        break;
                    }
                    AlgebraOperator::Division => {
                        let part = lhs.div(rhs);
                        if part.to_tex() != "0" {
                            parts.push(part.as_polynom_part());
                        }
                        parts.extend_from_slice(self.parts.get(i + 3..).unwrap_or(&[]));
                        break;
                    }
                    _ => {
                        parts.push(lhs.as_polynom_part());
                        parts.push(op.as_polynom_part());
                    }
                }
            }
        }
        let p = Polynom { parts };
        if p.parts
            .contains(&PolynomPart::Operator(AlgebraOperator::Multiplication))
            || p.parts
                .contains(&PolynomPart::Operator(AlgebraOperator::InvMulti))
            || p.parts
                .contains(&PolynomPart::Operator(AlgebraOperator::Division))
        {
            env_info(
                "operations",
                format!(
                    "simplify_mul_div does contain mul or div ops, doing an other iteration {}",
                    p.to_tex()
                ),
            );
            return p.simplify_mul_div();
        }

        env_info(
            "operations",
            format!("simplify_mut_div after {}", p.as_math().to_tex()),
        );
        p
    }

    //  AS - Addition and Subtraction (left-to-right)
    pub fn simplify_add_sub(&self) -> Polynom {
        env_info(
            "operations",
            format!("simplify_add_sub before {}", self.to_tex()),
        );

        let mut vec = self
            .to_vector()
            .to_based_matrix()
            .add_all()
            .as_polynom()
            .to_vector();

        let mut ret = vec.add_all().as_polynom();
        ret.sort();

        env_info(
            "operations",
            format!("simplify_add_sub after {}", ret.to_tex()),
        );
        ret
    }
    pub fn morph_ops(&self) -> Polynom {
        env_info("ops", format!("morph_ops before {}", self.to_tex()));

        let mut parts: Vec<PolynomPart> = Vec::new();
        let windows = self.parts.windows(2);
        let len = windows.len();

        let mut skip = false;
        let mut change = false;

        for (i, win) in windows.enumerate() {
            env_info("ops", format!("morph_ops checking {:#?}", win));
            if skip {
                env_info("ops", format!("morph_ops skipping this loop {:#?}", win));
                skip = false;
                continue;
            }
            match win {
                [PolynomPart::Operator(AlgebraOperator::Addition), PolynomPart::Operator(AlgebraOperator::Addition)]
                | [PolynomPart::Operator(AlgebraOperator::Subtraction), PolynomPart::Operator(AlgebraOperator::Subtraction)] =>
                {
                    env_info("ops", format!("morph_ops morphing {:#?}  to +", win));
                    parts.push(PolynomPart::Operator(AlgebraOperator::Addition));
                    skip = true;
                    change = true;
                    continue;
                }
                [PolynomPart::Operator(AlgebraOperator::Addition), PolynomPart::Operator(AlgebraOperator::Subtraction)]
                | [PolynomPart::Operator(AlgebraOperator::Subtraction), PolynomPart::Operator(AlgebraOperator::Addition)] =>
                {
                    env_info("ops", format!("morph_ops morphing {:#?}  to -", win));
                    parts.push(PolynomPart::Operator(AlgebraOperator::Subtraction));
                    skip = true;
                    change = true;
                    continue;
                }
                _ => {}
            }
            env_info(
                "ops",
                format!("morph_ops no ops, just adding {:#?}", win[0]),
            );
            if i != len {
                parts.push(win[0].clone());
            }
        }
        if let Some(m) = self.parts.last() {
            parts.push(m.clone());
        }

        let p = Polynom { parts };

        if change {
            env_info(
                "ops",
                format!(
                    "doing another iteration of morph_op after {}",
                    p.as_math().to_tex()
                ),
            );

            return p.morph_ops();
        }
        env_info(
            "operations",
            format!("morph_op after {}", p.as_math().to_tex()),
        );
        p
    }
}
