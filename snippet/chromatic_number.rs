// https://www.cs.helsinki.fi/u/mkhkoivi/publications/sicomp-2009.pdf
// O(2^n n)
// Note: this implementation is probabilisitic and therefore CAN FAIL.
fn chromatic_number(g: &[usize]) -> usize {
    let n = g.len();
    let mut dp = vec![MInt::new(0); 1 << n];
    for bits in (0..(1 << n) - 1).rev() {
        for i in 0..n {
            if (bits & 1 << i) == 0 {
                dp[bits] = dp[bits | 1 << i] + dp[bits | g[i] | 1 << i] + 1;
                break;
            }
        }
    }
    let mut cur = vec![MInt::new(0); 1 << n];
    for bits in 0usize..1 << n {
        if bits.count_ones() % 2 == 0 {
            cur[bits] += 1;
        } else {
            cur[bits] -= 1;
        }
    }
    for i in 0..n + 1 {
        let mut tot = MInt::new(0);
        for bits in 0usize..1 << n {
            tot += cur[bits];
            cur[bits] *= dp[bits];
        }
        if tot != MInt::new(0) {
            return i;
        }
    }
    unreachable!();
}
