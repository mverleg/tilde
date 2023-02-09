/// Included from build.rs

/// Estimate the number of letters needed to encode the uint argument.
pub fn encode_snippet_len_estimate(nr: u64) -> usize {
    let mut length: usize = 1;
    let opener_n: u64 = 7;  // must be in sync with var_uint.rs
    let follow_2n: u64 = 16;  // must be in sync with var_uint.rs
    let follow_1n = follow_2n / 2;
    let mut non_close_letter_cnt_doubled = 0;
    let mut rem = nr / opener_n;
    while rem > 0 {
        rem -= 1;
        let block_extra = non_close_letter_cnt_doubled / 2;
        length += block_extra + 1;
        let div = follow_1n * follow_2n.pow(block_extra as u32);
        rem = rem / div;
        non_close_letter_cnt_doubled += 1;
    }
    return length;
}
