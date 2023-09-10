# ZKSummit10 - Exploring Noir, Circom, halo2 Workshop

## Prompt
Write Noir, Halo2 and Circom circuits that constrain integer division i.e decimals are truncated. For example 12 / 5 = 2

Inputs: x, y
Outputs: quotient

## Setup
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
3. Copy circom code to https://zkrepl.dev/. zkrepl is a great tool to prototype and is faster than installing circom and snarkjs locally