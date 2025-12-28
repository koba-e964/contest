fn calc(n: usize, bits: u64) -> bool {
    let mut dp = vec![vec![[0; 2]; n]; n];
    for i in 0..n {
        for j in 0..n {
            let idx = (bits >> (i * n + j)) & 1;
            let mut me = [0; 2];
            me[idx as usize] += 1;
            if i > 0 {
                for b in 0..2 {
                    me[b] += dp[i - 1][j][b];
                }
            }
            if j > 0 {
                for b in 0..2 {
                    me[b] += dp[i][j - 1][b];
                }
            }
            dp[i][j] = me;
        }
    }
    let mut ans = [0; 2];
    for i in 0..n {
        for j in 0..n {
            let idx = (bits >> (i * n + j)) & 1;
            ans[idx as usize] += dp[i][j][idx as usize] - 1;
        }
    }
    ans[0] == ans[1]
}

fn main() {
    for n in 3..=7 {
        if n % 2 == 0 { continue; }
        let mut mi = (n * n + 1, 0);
        for bits in 0..1u64 << (n * n) {
            let val = (bits ^ (bits.reverse_bits() >> (64 - n * n)) ^ ((1u64 << (n * n)) - 1)).count_ones() as usize;
            if val < mi.0 && calc(n, bits) {
                println!();
                for i in 0..n {
                    for j in 0..n {
                        print!("{}", (bits >> (i * n + j)) & 1);
                    }
                    println!();
                }
                mi = std::cmp::min(mi, (val, bits));
            }
        }
    }
}
