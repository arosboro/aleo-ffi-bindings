crate::traits::PrimeField;

pub struct LinearCombination<F: PrimeField> {
    constant: F,
    terms: IndexMap<Variable<F>, F>,
    /// The value of this linear combination, defined as the sum of the `terms` and `constant`.
    value: F,
}
