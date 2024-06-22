# JavaScript Raydium AMM V4 Log Decoder
## Overview
JavaScript (nodejs) wrapper around Raydium AMM V4 Rust SDK for decoding Solana transaction logs containing a ray_log.<br><br>
## Ability to decode:
* Raydium swap logs: swap amount in, swap amount out, swap direction.
* Raydium liquidity add logs: SOL amount in, coin amount in, token decimals.
## How to use:
* Run ```npm install``` inside ```/node_call```.
* Run ```cargo build``` inside ```/program```.
* ```node_call/index.js``` contains JavaScript functions for decoding Raydium logs.
* ```program/src/main.rs``` calls the respective decode functions natively.
* Example usage: ```decode_ray_log_swap_amount_in('<RAY_LOG HERE>');```, more examples provided in index.js.
* Ability to export log decoding functions to outside JS files, use absolute path for this, instructions in index.js.

## Additional Notes:
* The wrapper does not have error checking, functions with "swap" must have a Raydium "swap" log as input and functions with "init" must have a Raydium "init"/"lp add" log as input.