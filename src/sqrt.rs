//! This module provides multiple methods of computing a square root

/// Approximate the square root of a positive number using newton's method
///
/// `difference` is the allowed/negligible absolute difference between the approximated square root and the
/// actual square root
///
/// doesn't work for numbers larger than 1e154, because f64 is not enough for the calculation
pub fn newtons_method(square: f64, difference: f64) -> f64 {
    let mut guess = 1_f64;
    let mut previous_guess = 0.0; 

    while difference < (guess.powi(2) - square).abs() && guess != previous_guess {
        previous_guess = guess;
        guess -= (guess.powi(2) - square) / (2.0 * guess);
    }

    guess
}
