use std::{collections::HashMap, str::FromStr};
use num_bigint::BigUint;
use num_format::{Locale, ToFormattedString};

/// This algorithm will compute the length of the Collatz sequence for all numbers in a given range.
/// It will then output:
///     - The number in the provided range with the longest Collatz sequence.
///     - The biggest number computed while computing the Collatz sequence.
/// 
/// # Collatz Sequence
/// The Collatz sequence is defined as follows:
///     - If the number is even, divide it by 2.
///     - If the number is odd, multiply it by 3 and add 1.
/// And you repeat this process until you reach 1. The length of the sequence is the number of steps needed to reach 1.
/// 
/// The Collatz conjecture is the conjecture that for any number, the Collatz sequence will always reach 1.
/// Hypothetically, the sequence could go on forever, or find another cycle, as this conjecture has not yet been proven.
fn main() {
    // create a new HashMap that will cache the length of each Collatz sequence
    // this way if a sequence ends up somewhere we know, we don't have to recompute everything
    let mut cache = HashMap::new();
    cache.insert(BigUint::from(1u32), 0);

    // define where the sequence should start and end
    let mut number = BigUint::from(2u32);
    let target = BigUint::from_str("10_000_000").unwrap();

    // loop every number in the the range
    loop {
        // compute the sequence for the current number
        collatz_sequence(&mut cache, number.clone());

        // get the next number
        number += 1u32;

        // check if we reached the target
        if target <= number {
            break;
        }
    }

    // display the results to the user
    let max_steps = cache
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    let max_computed = cache
        .iter()
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .unwrap();

    println!("In the range [1-{target}], {number} has the most number of steps : {steps}.",
        target = target.to_formatted_string(&Locale::en),
        number = max_steps.0.to_formatted_string(&Locale::en),
        steps = max_steps.1
    );
    println!("While computing the sequence for the numbers in the range [1-{target}], the biggest number computed was {number}.",
        target = target.to_formatted_string(&Locale::en),
        number = max_computed.0.to_formatted_string(&Locale::en)
    );
}

/// Compute the collatz sequence for a given number and add every step to the cache.
/// This function will use the cache to avoid computing the same sequence twice.
fn collatz_sequence(cache: &mut HashMap<BigUint, usize>, mut number: BigUint) {
    // a list of the numbers we are going through while computing the sequence
    let mut temps_list = vec![number.clone()];

    // compute the sequence for that number
    while !cache.contains_key(&number) {
        number = collatz_step(number);
        temps_list.push(number.clone());
    }

    let mut steps = cache[&number];
    for number in temps_list.into_iter().rev().skip(1) {
        steps += 1;
        cache.insert(number, steps);
    }
}

/// Compute the next number in the collatz sequence.
/// We use a BigUint because the numbers can get very big very quickly.
fn collatz_step(number: BigUint) -> BigUint {
    // if the number is even, divide it by 2
    if number.clone() % 2u32 == BigUint::from(0u32) {
        number / 2u32

    // if the number is odd, multiply it by 3 and add 1
    } else {
        number * 3u32 + 1u32
    }
}
