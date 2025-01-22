pub fn digits_to_fractional_f32(n: u32) -> f32 {
    if n == 0 {
        return 0.0;
    }

    // Calculate the number of digits in the n
    let digits = (n as f64).log10().floor() as u32 + 1;

    // Divide by 10^digits to scale correctly
    n as f32 / 10_f32.powi(digits as i32)
}
