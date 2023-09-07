mod maths;

use rayon::prelude::*;
use crate::maths::{is_prime_number, stress_cpu_avx2};

pub fn run() {
    loop {
        (0..1000000).into_par_iter().for_each(|number| {
            is_prime_number(number);
        });
    }
}

pub fn run_avx2() {
    loop {
        (0..1000000).into_par_iter().for_each(|_| {
            stress_cpu_avx2();
        });
    }
}

fn main() {
    let cpu_thread_count: usize = num_cpus::get();
    println!("Running CPU stress test on {} threads! (press CTRL+C to exit)", cpu_thread_count);

    if is_x86_feature_detected!("avx2") {
        println!("Detected AVX2!");

        run_avx2();
    } else {
        run();
    }
}
