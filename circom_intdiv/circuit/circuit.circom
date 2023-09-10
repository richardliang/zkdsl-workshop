pragma circom 2.1.4;

include "circomlib/comparators.circom";

template IntegerDivision () {
    signal input x;
    signal input y;
    signal output quotient;
    
    // TODO: implement integer division in Circom!
}

component main { public [ x ] } = IntegerDivision();

/* INPUT = {
    "x": "12",
    "y": "5"
} */