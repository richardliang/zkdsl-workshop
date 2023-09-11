# ZKSummit10 - Exploring Noir, Circom, halo2 Workshop

## Prompt
Write Noir, Halo2 and Circom circuits that constrain integer division i.e decimals are truncated. For example 12 / 5 = 2

Inputs: x, y where x and y are nonnegative integers < 2^32 bits

Outputs: quotient

## Instructions
1. Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install Nargo (Noir)
```
curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash
```
Close terminal, and run 
```
noirup
```
3. Fork this repo and `git clone`
4. From root`cd circom_intdiv` and copy circom code into https://zkrepl.dev/. zkrepl is a great tool to prototype and is faster than installing circom and snarkjs locally
5. From root `cd halo2_intdiv` and fill the logic in `TODO`. To test your solution, run `cargo test`
6. From root `cd noir_intdiv` and fill the logic in `TODO`. To test your solution, run `nargo test`