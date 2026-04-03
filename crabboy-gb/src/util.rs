#[macro_export]
macro_rules! nth_bit {
    // Base case: If n is 0, return the least significant bit (LSB)
    ($num:expr, 0) => {
        $num & 1
    };

    // Recursive case: Shift the number to the right by n bits and
    // perform bitwise AND with 1 to get the nth bit
    ($num:expr, $n:expr) => {
        ($num >> $n) & 1
    };
}
