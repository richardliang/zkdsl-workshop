// Implement integer division in Noir!
//
// Public Inputs:
// x: Field element
// y: Field element
// 
// Public Output: quotient Field element from integer division
// 
// Note: don't change the function signature / types below. This is for exercise purposes!

// WAY 1
fn main(x : pub Field, y : pub Field) -> pub Field {
    // TODO: implement!
    // ...

    let res = x as u32 / y as u32;
    res as Field
}

// WAY 2: Use unconstrained functions (<-- in circom, native Rust in halo2)
// fn main(x : Field, y : pub Field) -> pub Field {
//     let (quo, rem) = unconstrained_div(x, y);
//     assert(quo * y + rem == x);
//     let _unusedRem = rem as u32;
//     let _unusedQuo = quo as u32;
//     quo
// }

// unconstrained fn unconstrained_div(x : Field, y : Field) -> (Field, Field) {
//     let a = x as u32 / y as u32;
//     let b = x as u32 % y as u32;
//     (a as Field, b as Field)
// }

// WAY3: Changing functions signature
// fn main(x : u32, y : pub u32) -> pub u32 {
//     x / y
// }

#[test]
fn test_main() {
    // 12 / 5 = 2
    let res = main(12, 5);

    assert(res == 2);
}
