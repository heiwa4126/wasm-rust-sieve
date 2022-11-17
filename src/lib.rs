extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sieve(max_count: usize) -> Vec<usize> {
    let max = max_count + 1;
    let mut primes: Vec<bool> = Vec::with_capacity(max);
    let mut result: Vec<usize> = vec![];

    for _ in 1..=max {
        primes.push(true);
    }

    for i in 2..max {
        if primes[i] {
            result.push(i);

            let mut j = i;
            while j * i < max {
                primes[j * i] = false;
                j += 1;
            }
        }
    }
    return result;
}
