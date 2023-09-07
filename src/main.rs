use rayon::prelude::*;

pub fn is_prime_number(number: i32) -> bool {
    let is_even: bool = (number & 1) == 0;
    let is_divisible: bool = (number % 3) == 0;

    let is_not_prime: bool = is_even || is_divisible;

    let mut counter: i32 = 5;
    let mut increment: i32 = 2;
    let mut is_prime: bool = true;

    while counter * counter <= number {
        let is_divisible: bool = (number % counter) == 0;
        is_prime &= !is_divisible;
        increment = 6 - increment;
        counter += increment;
    }

    return number > 1 && is_prime && !is_not_prime;
}

pub fn run() {
    println!("Warming up CPU...");

    loop {
        (0..1000000).into_par_iter().for_each(|number| {
            is_prime_number(number);
        });
    }
}

pub fn run_avx2() {
    println!("Warming up CPU...");
}

fn main() {
    println!("CPU stress test (press CTRL+C to exit)");

    if !is_x86_feature_detected!("avx2") {
        println!("Detected AVX2!");

        run_avx2();
    } else {
        run();
    }
}
