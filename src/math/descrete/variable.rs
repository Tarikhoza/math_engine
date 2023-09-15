use crate::math::algebra::variable::Variable;
use crate::math::descrete::Descrete;
use crate::parser::ParsablePrimitiveAsVariable;

use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

impl Descrete for Variable {
    fn is_prime(&self) -> bool {
        if self.value <= dec!(1) {
            return false;
        }
        let n = self.value;
        let mut i = dec!(2);
        while (i < n) {
            if (n % i).is_zero() {
                return false;
            }
            i += dec!(1);
        }
        true
    }

    fn primes_up_to(&self) -> Vec<Variable> {
        let mut primes: Vec<Variable> = vec![];
        if !self.is_integer() {
            return primes;
        }
        if self.is_prime() {
            primes.push(self.clone());
            return primes;
        }
        let n = self.value;
        let mut i = dec!(2);
        while (i < n) {
            let x = i.as_variable();
            if x.is_prime() {
                primes.push(x);
            }
            i += dec!(1);
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

    fn lowest_common_denominator(&self, other: &Variable) -> Variable {
        for i in self.factorise_prime() {
            for j in other.factorise_prime() {
                if i.value == j.value {
                    return i;
                }
            }
        }
        1.as_variable()
    }

    fn is_divisable(&self, other: &Variable) -> bool {
        if self.is_integer() && other.is_integer() && other.value != dec!(0) {
            return (self.value % other.value).is_zero();
        }
        false
    }
}
