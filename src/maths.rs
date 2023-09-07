use std::arch::x86_64::{__m256i, _mm256_add_epi32, _mm256_loadu_si256, _mm256_mullo_epi32, _mm256_storeu_si256, _mm256_sub_epi32};

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

pub fn stress_cpu_avx2() -> [i32; 8] {
    let data1: [i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let data2: [i32; 8] = [8, 7, 6, 5, 4, 3, 2, 1];

    // Create AVX2 vectors
    let avx2_vector1: __m256i = unsafe { _mm256_loadu_si256(data1.as_ptr() as *const __m256i) };
    let avx2_vector2: __m256i = unsafe { _mm256_loadu_si256(data2.as_ptr() as *const __m256i) };

    // Perform multiple AVX2 calculations
    let result1: __m256i = unsafe { _mm256_add_epi32(avx2_vector1, avx2_vector2) };
    let result2: __m256i = unsafe { _mm256_sub_epi32(avx2_vector1, avx2_vector2) };
    let result3: __m256i = unsafe { _mm256_mullo_epi32(avx2_vector1, avx2_vector2) };

    // Combine the results as needed (e.g., addition here)
    let final_result: __m256i = unsafe { _mm256_add_epi32(result1, _mm256_add_epi32(result2, result3)) };

    // Store the final result in memory
    let mut result_data: [i32; 8] = [0; 8];
    unsafe {
        _mm256_storeu_si256(result_data.as_mut_ptr() as *mut __m256i, final_result);
    }

    result_data
}