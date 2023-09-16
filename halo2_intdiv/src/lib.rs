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
    _marker: PhantomData<F>,
}

impl <'range, F: ScalarField> IntegerDivisionChip<F> {
    pub fn new(range: RangeChip<F>) -> Self {
        Self {
            range,
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
        
        // Calculate quotient and remainder using native Rust
        let quo = x / y;
        let rem = x % y;

        // Assign calculated values to the table
        let x = ctx.load_witness(F::from(x as u64));
        let quo = ctx.load_witness(F::from(quo as u64));
        let rem = ctx.load_witness(F::from(rem as u64));

        // check quo and rem are both < 32 bits
        // quo * y + rem = x
        // rem in [0, quo)
        self.range.range_check(ctx, rem, 32);
        self.range.range_check(ctx, quo, 32);
        
        // MALICIOUS PROVER EXAMPLE
        // 1. Comment out quo range check
        // 2. Uncomment below
        // 3. Comment out Rust assert check in test
        // let rem = F::from(0);
        // let quo = *x.value() * F::from(y as u64).invert().unwrap();
        // let quo = ctx.load_witness(F::from(quo));
        // let rem = ctx.load_witness(F::from(rem));

        // Reconstruct x from quotient and remainder and constrain
        let reconstructed_x = self.range.gate().mul_add(ctx, quo, Constant(F::from(y as u64)), rem);
        ctx.constrain_equal(&x, &reconstructed_x);

        quo
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use halo2_base::utils::testing::base_test;
    use halo2_base::halo2_proofs::halo2curves::bn256::Fr;

    #[test]
    fn test_integer_division() {
        env_logger::init();
        std::env::set_var("RUST_LOG", true.to_string());

        let k = 6;
        
        // Circuit inputs
        let x = 12;
        let y = 5;
        
        base_test().k(k as u32).lookup_bits(k - 1).run(|ctx, range| {
            let range = range.clone();
            let chip = IntegerDivisionChip::<Fr>::new(range);

            let result = chip.integer_division(
                ctx,
                x,
                y
            );
    
            let expected_result = 12 / 5;
            
            // Run test
            assert_eq!(result.value(), &Fr::from(expected_result));
        });
    }
}