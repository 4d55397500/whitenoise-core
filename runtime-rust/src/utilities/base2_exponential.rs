use whitenoise_validator::errors::*;
// use probability::distribution::{Gaussian, Laplace, Inverse, Distribution};
// use ieee754::Ieee754;
use std::{cmp, f64::consts, f64::MAX, char};
use rug::rand::{ThreadRandGen, ThreadRandState};
use rug::Float;
use math::round;

use crate::utilities::utilities;

/// Implementation of the `get_random_value` function from Ilv19.
///
/// Generate uniform numbers from the interval `[0, 2^(pow_2))`.
/// `pow_2` and `precision` are equivalent to `start_pow`+1 and `p` from the original code.
///
/// # Arguments
/// * `pow_2` - The power of two that will be an upper bound on the generated values.
/// * `precision` - Bits of precision with which you want the output generated.
///
/// # Example
/// ```
/// use whitenoise_runtime::utilities::base2_exponential::sample_uniform_bounded_pow_2;
/// let unif = sample_uniform_bounded_pow_2(3, 52).to_f64();
/// assert!(unif >= 0. && unif < 8.);
/// ```
pub fn sample_uniform_bounded_pow_2(pow_2: i64, precision: u32) -> Float {
    // get random bits from OpenSSL and convert them into a vector of ints
    let n_bytes = round::ceil((precision as f64 / 8.) as f64, 0) as usize;
    let bits = utilities::get_bytes(n_bytes);
    let bit_vec: Vec<u32> = bits.chars().map(|x| x.to_digit(10).unwrap()).collect();

    // loop over first `precision` bits and store the number it represents (bit * 2^k) for some k
    let result_vec: Vec<Float> = (0..precision).map(|i|
                                                       Float::with_val(precision,
                                                            Float::with_val(precision, bit_vec[i as usize]) *
                                                            Float::with_val(precision, 2_f64.powi((pow_2 - 1 - (i as i64)) as i32))
                                                       ) ).collect();

    // sum over vector to get final uniform output
    let unif = Float::with_val(precision, Float::sum(result_vec.iter()));

    return unif
}

/// Slightly altered implementation of the `randomized_round` function from Ilv19.
///
/// Rounds the input x to an adjacent integer value.
/// x is rounded up with probability `x - floor(x)` and
/// rounding randomness is sampled at the level of `precision`
///
/// This implementation differs from the original in that `u_min` and `u_max` are
/// replaced by explicit arguments `min_return` and `max_return`. Because `min_return`
/// and `max_return` might be `f64` for some other use case, we allow the to be so here,
/// even `u_min` and `u_max` are `i64` in the original code.
///
/// Additionally, the original code assumes that lower utilities are better than higher ones, so
/// `u_min` > `u_max`. We take the approach of higher utilities being better.
///
/// # Arguments
/// * `x` - Element to be rounded.
/// * `precision` - Bits of precision with which you want the output generated.
/// * `min_return` - Minimum allowable return value.
/// * `max_return` - Maximum allowable return value.
///
/// # Example
/// use whitenoise_runtime::utilities::base2_exponential::randomized_round;
/// let rounded = randomized_round(3.5, 52, 2., 5.);
/// assert!(rounded == 3. || rounded == 4.)
pub fn randomized_round(x: f64, precision: u32, min_return: f64, max_return: f64) -> f64 {
    let unif = sample_uniform_bounded_pow_2(0, precision);

    let lower = round::floor(x, 0) as f64;
    let upper = round::ceil(x, 0) as f64;

    if unif > x - (lower as f64) {
        return lower.max(min_return).min(max_return);
    } else {
        return upper.min(max_return).max(min_return);
    }
}
/// Implementation of `normalized sample` from the original code.
///
/// Returns an index based on weights.
///
/// # Arguments
/// * `weights` - Weights for each index.
/// * `precision` - Bits of precision with which you want the output generated.
///
/// # Return
/// Index based on sampling weights.
///
/// # Example
pub fn normalized_sample(weights: Vec<f64>, precision: u32) -> usize {
    // generate Float version of weights
    let weights_Float: Vec<Float> = weights.iter().map(|x| Float::with_val(52, x)).collect();

    // get total weight
    let total_weight = Float::with_val(52, Float::sum(weights_Float.iter()));

    // generate cumulative weights
    let mut cumulative_weight_vec: Vec<rug::Float> = Vec::with_capacity(weights.len() as usize);
    for i in 0..weights.len() {
        cumulative_weight_vec.push( Float::with_val(53, Float::sum(weights_Float[0..(i+1)].iter())) );
    }

    // get maximum power of two needed for sampling
    let mut pow_2: i64 = 0;
    while (Float::with_val(52, pow_2)).exp2() > total_weight.to_f64() {
        pow_2 = pow_2 - 1;
    }
    while (Float::with_val(52, pow_2)).exp2() <= total_weight.to_f64() {
        pow_2 = pow_2 + 1;
    }

    // sample a random number from [0, 2^pow_2)
    let mut s = Float::with_val(52, std::f64::MAX);
    while s > total_weight {
        s = sample_uniform_bounded_pow_2(pow_2, precision);
    }

    // return the associated elements
    let mut index = 0;
    for i in 0..weights.len() {
        if cumulative_weight_vec[i].to_f64() >= s {
            index = i;
            break;
        }
    }
    return index;
}

