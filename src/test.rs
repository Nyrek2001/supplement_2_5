#[cfg(test)]
mod tests {
    use super::*;

    // Test for function that converts 32-bit float to bytes
    #[test]
    fn test_float_to_bytes() {
        let float_value: f32 = 3.14159;
        let result = float_to_bytes(float_value);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 64); // expected first byte of 3.14159
    }

    // Test for function that calls a function pointer
    #[test]
    fn test_function_pointer() {
        let add_fn: fn(i32) -> i32 = add_one;
        let result = call_function(add_fn, 5);
        assert_eq!(result, 6);
    }

    // Test for function that extracts specific bytes from a 64-bit float
    #[test]
    fn test_float64_to_bytes() {
        let float64_value: f64 = 3.141592653589793;
        let result = float64_to_bytes(float64_value);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 9); // expected byte from the float64 representation
    }
}
