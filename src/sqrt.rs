//! This module provides multiple methods of computing a square root

use crate::utility;

/// Approximate the square root of a positive number using newton's method
///
/// doesn't work for numbers larger than 1e154, because [`f64`] is not large enough for the calculation
pub fn newtons_method(square: f64) -> f64 {
    let mut guess = square;
    let mut previous_guess = 0.0;

    while !utility::close(guess, previous_guess, 1e-9) {
        previous_guess = guess;
        guess -= (guess.powi(2) - square) / (2.0 * guess);
    }

    guess
}
