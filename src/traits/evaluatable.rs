use std::ops::Add;

use crate::definitions;

trait Evaluatable {
    type Output;

    fn evaluate(self) -> Self::Output;
}

impl<Value> Evaluatable for definitions::Number<Value> {
    type Output = Value;

    fn evaluate(self) -> Value {
        self.value
    }
}

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

#[cfg(test)]
mod tests {
    use super::Evaluatable;
    use crate::definitions::*;

    #[test]
    fn test_addition() {
        assert_eq!(
            Addition {
                augend: Addition {
                    augend: Number { value: 1 },
                    addend: Number { value: 2 },
                },
                addend: Addition {
                    addend: Number { value: 3 },
                    augend: Number { value: 4 },
                }
            }.evaluate(),
            10
        );
    }
}
