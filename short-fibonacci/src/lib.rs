/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    Vec::new()
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    vec.reserve_exact(count);
    for _ in 0..count {
        vec.push(0)
    }
    vec
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let size = 5;
    let mut vec = create_buffer(size);
    for i in 0..size {
        vec[i] = fibonacci_impl(i)
    }
    vec
}

fn fibonacci_impl(index: usize) -> u8 {
    if index <= 1 {
        1
    }
    else {
        fibonacci_impl(index - 1) + fibonacci_impl(index - 2)
    }
}
