use halo2_base::{self, QuantumCell::Constant};
use halo2_base::{
    gates::{GateInstructions, RangeChip, RangeInstructions},
    utils::ScalarField,
    AssignedValue, Context,
};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct IntegerDivisionChip<F: ScalarField> {
    pub range: RangeChip<F>,
    pub lookup_bits: usize,
    _marker: PhantomData<F>,
}

impl <'range, F: ScalarField> IntegerDivisionChip<F> {
    pub fn new(lookup_bits: usize) -> Self {
        let range = RangeChip::<F>::default(lookup_bits);

        Self {
            range,
            lookup_bits,
            _marker: PhantomData,
        }
    }

    // Implement integer division in Halo2!
    //
    // Public Inputs:
    // x: u32
    // y: u32
    // 
    // Public Output: quotient from integer division
    pub fn integer_division(
        &self,
        ctx: &mut Context<F>,
        x: u32,
        y: u32,
    ) -> AssignedValue<F> {
        // TODO: implement!
        // ...
        
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use halo2_base::gates::builder::{GateThreadBuilder, RangeCircuitBuilder};
    use halo2_base::halo2_proofs::{halo2curves::bn256::Fr, dev::MockProver};

    #[test]
    fn test_integer_division() {
        env_logger::init();

        let k = 16;
        // Configure builder
        let mut builder = GateThreadBuilder::<Fr>::mock();
        let lookup_bits = 8;
        // NOTE: Need to set var to load lookup table
        std::env::set_var("LOOKUP_BITS", lookup_bits.to_string());
        
        // Circuit inputs
        let x = 12;
        let y = 5;

        // Configure black scholes chip
        let chip = IntegerDivisionChip::<Fr>::new(lookup_bits);
        
        let result = chip.integer_division(
            builder.main(0),
            x,
            y
        );

        let expected_result = 12 / 5;

        // Run test
        assert_eq!(result.value(), &Fr::from(expected_result));

        // Minimum rows is the number of rows used for blinding factors
        // This depends on the circuit itself, but we can guess the number and change it if something breaks (default 9 usually works)
        builder.config(k, Some(9));
        // Create mock circuit
        let circuit = RangeCircuitBuilder::mock(builder);
        
        // Run mock prover to ensure output is correct
        MockProver::run(k as u32, &circuit, vec![]).unwrap().assert_satisfied();
    }
}