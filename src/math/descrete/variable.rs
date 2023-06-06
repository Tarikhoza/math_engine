use crate::math::algebra::variable::Variable;
use crate::math::descrete::Descrete;
use crate::parser::ParsableGenericsAsVariable;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl Descrete for Variable {
    fn is_prime(&self) -> bool {
        if self.value <= dec!(1) {
            return false;
        }
        let n = self.value.to_i64().expect("converting to i64 failed");
        for i in 2..=(n as f64).sqrt() as i64 {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    fn primes_up_to(&self) -> Vec<Variable> {
        let mut primes: Vec<Variable> = vec![];
        for i in 2..self.value.to_i64().expect("converting to i64 failed") {
            let x = i.as_variable();
            if x.is_prime() {
                primes.push(x);
            }
        }
        primes
    }

    fn factorise_prime(&self) -> Vec<Variable> {
        let mut math = self.clone();
        if math.is_prime() {
            return vec![math];
        }
        let mut factorised: Vec<Variable> = vec![];
        for i in math.primes_up_to() {
            if math.value.is_zero() {
                break;
            }
            while ((math.value % i.value).is_zero()) {
                math.value /= i.value;
                factorised.push(i.clone());
            }
        }
        factorised
    }

    fn lowest_common_denominator(&self, other: Variable) -> Variable {
        //TODO use generators for generating primes
        for i in self.factorise_prime() {
            for j in other.factorise_prime() {
                if i.value == j.value {
                    return i;
                }
            }
        }
        return 1.as_variable();
    }
}
