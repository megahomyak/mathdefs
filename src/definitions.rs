pub struct Number<Value> {
    pub value: Value,
}
pub struct Addition<Augend, Addend> {
    pub augend: Augend,
    pub addend: Addend,
}
pub struct Subtraction<Minuend, Subtrahend> {
    pub minuend: Minuend,
    pub subtrahend: Subtrahend,
}
pub struct Multiplication<Multiplier, Multiplicand> {
    pub multiplier: Multiplier,
    pub multiplicand: Multiplicand,
}
pub struct Division<Dividend, Divisor> {
    pub dividend: Dividend,
    pub divisor: Divisor,
}
pub struct Sine<Angle> {
    pub angle: Angle,
}
pub struct Cosine<Angle> {
    pub angle: Angle,
}
pub struct Tangent<Angle> {
    pub angle: Angle,
}
pub struct Cotangent<Angle> {
    pub angle: Angle,
}
pub struct Exponentiation<Base, Exponent> {
    pub base: Base,
    pub exponent: Exponent,
}
pub struct Logarithm<Base, Antilogarithm> {
    pub base: Base,
    pub antilogarithm: Antilogarithm,
}
