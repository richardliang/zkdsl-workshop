pragma circom 2.1.4;

include "circomlib/comparators.circom";

template IntegerDivision () {
    signal input x;
    signal input y;
    signal output quotient;
    
    quotient <-- x \ y;
    signal rem;
    rem <-- x % y;

    component ltrem = LessThan(32);
    ltrem.in[0] <== quotient;
    ltrem.in[1] <== x;

    component ltquo = LessThan(32);
    ltquo.in[0] <== rem;
    ltquo.in[1] <== y;

    x === quotient * y + rem;
}

component main { public [ x ] } = IntegerDivision();

/* INPUT = {
    "x": "12",
    "y": "5"
} */