use std::ops::{Add, Mul};

use crate::definitions;

trait Evaluatable {
    type Output;

    fn evaluate(self) -> Self::Output;
}

macro_rules! evaluatable_primitive {
    ($t:ty) => {
        impl Evaluatable for $t {
            type Output = Self;

            fn evaluate(self) -> Self::Output {
                self
            }
        }
    };
}

evaluatable_primitive!(i8);
evaluatable_primitive!(i16);
evaluatable_primitive!(i32);
evaluatable_primitive!(i64);
evaluatable_primitive!(i128);
evaluatable_primitive!(u8);
evaluatable_primitive!(u16);
evaluatable_primitive!(u32);
evaluatable_primitive!(u64);
evaluatable_primitive!(u128);
evaluatable_primitive!(isize);
evaluatable_primitive!(usize);
evaluatable_primitive!(f32);
evaluatable_primitive!(f64);

impl<Augend, Addend, EvaluatedAugend, EvaluatedAddend, Output> Evaluatable
    for definitions::Addition<Augend, Addend>
where
    Augend: Evaluatable<Output = EvaluatedAugend>,
    Addend: Evaluatable<Output = EvaluatedAddend>,
    EvaluatedAddend: Add<EvaluatedAugend, Output = Output>,
{
    type Output = Output;

    fn evaluate(self) -> Self::Output {
        self.addend.evaluate() + self.augend.evaluate()
    }
}

impl<Multiplier, Multiplicand, EvaluatedMultiplier, EvaluatedMultiplicand, Output> Evaluatable
    for definitions::Multiplication<Multiplier, Multiplicand>
where
    Multiplier: Evaluatable<Output = EvaluatedMultiplier>,
    Multiplicand: Evaluatable<Output = EvaluatedMultiplicand>,
    EvaluatedMultiplicand: Mul<EvaluatedMultiplier, Output = Output>,
{
    type Output = Output;

    fn evaluate(self) -> Self::Output {
        self.multiplicand.evaluate() * self.multiplier.evaluate()
    }
}

#[cfg(test)]
mod tests {
    use super::Evaluatable;
    use crate::definitions::*;

    #[test]
    fn test_addition() {
        assert_eq!(
            Addition {
                augend: Addition {
                    augend: 1,
                    addend: 2,
                },
                addend: Addition {
                    addend: 3,
                    augend: 4,
                }
            }
            .evaluate(),
            10
        );
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(
            Multiplication {
                multiplier: Multiplication {
                    multiplier: 2,
                    multiplicand: 3,
                },
                multiplicand: Multiplication {
                    multiplicand: 4,
                    multiplier: 5,
                },
            }
            .evaluate(),
            120
        );
    }
}
