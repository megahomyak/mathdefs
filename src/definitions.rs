pub struct Number<Value> {
    value: Value,
}
pub struct Addition<Augend, Addend> {
    augend: Augend,
    addend: Addend,
}
pub struct Subtraction<Minuend, Subtrahend> {
    minuend: Minuend,
    subtrahend: Subtrahend,
}
pub struct Multiplication<Multiplier, Multiplicand> {
    multiplier: Multiplier,
    multiplicand: Multiplicand,
}
pub struct Division<Dividend, Divisor> {
    dividend: Dividend,
    divisor: Divisor,
}
pub struct Sine<Angle> {
    angle: Angle,
}
pub struct Cosine<Angle> {
    angle: Angle,
}
pub struct Tangent<Angle> {
    angle: Angle,
}
pub struct Cotangent<Angle> {
    angle: Angle,
}
pub struct Exponentiation<Base, Exponent> {
    base: Base,
    exponent: Exponent,
}
pub struct Logarithm<Base, Antilogarithm> {
    base: Base,
    antilogarithm: Antilogarithm,
}
