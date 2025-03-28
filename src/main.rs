/// Converts a 32-bit floating point number to an array of 4 bytes.
///
/// # Parameters
/// - `f`: A 32-bit floating point number (`f32`).
///
/// # Returns
/// An array of 4 bytes (`[u8; 4]`).
///
/// # Safety
/// This function uses `unsafe` to transmute a floating point number into raw bytes.
/// You must ensure that this operation is safe in the context of the application.
pub fn float_to_bytes(f: f32) -> [u8; 4] {
    // We use unsafe to directly interpret the float as raw bytes
    let bytes: [u8; 4] = unsafe { std::mem::transmute(f) };
    bytes
}


/// A simple function that adds 1 to the provided integer.
/// This function is used as a function pointer in the `call_function` example.
///
/// # Parameters
/// - `x`: An integer value to which 1 will be added.
///
/// # Returns
/// The integer `x + 1`.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Calls the provided function pointer with a single 32-bit integer parameter.
///
/// # Parameters
/// - `func`: A function pointer that takes an integer and returns an integer.
/// - `val`: The 32-bit integer to be passed to `func`.
///
/// # Returns
/// The result of calling `func` on `val`.
pub fn call_function(func: fn(i32) -> i32, val: i32) -> i32 {
    func(val)
}

/// Extracts specific bytes (at indexes 0, 2, 4, and 6) from the 64-bit floating point number.
///
/// # Parameters
/// - `f`: A 64-bit floating point number (`f64`).
///
/// # Returns
/// An array of 4 bytes (`[u8; 4]`) from the 64-bit floating point number.
pub fn float64_to_bytes(f: f64) -> [u8; 4] {
    let bytes: [u8; 8] = unsafe { std::mem::transmute(f) };

    // We return bytes at indexes 0, 2, 4, and 6
    [
        bytes[0],
        bytes[2],
        bytes[4],
        bytes[6],
    ]
}
