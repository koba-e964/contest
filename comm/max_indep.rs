/// Verified by https://atcoder.jp/contests/code-thanks-festival-2017/submissions/4151536
fn max_indep(g: &[Vec<bool>]) -> usize {
    fn max_indep_internal(g: &[i64], mut used: usize,
                          mut remaining: i64, cur_ma: &mut usize) {
        let n = g.len();
        let mut degma = (0, 0);
        // preprocessing: eliminate all vertices with degree <= 1
        // find the max degree
        let mut updated = true;
        while updated {
            updated = false;
            degma = (0, 0);
            for i in 0..n {
                if (remaining & 1 << i) == 0 { continue; }
                let deg = (g[i] & remaining).count_ones();
                degma = std::cmp::max(degma, (deg + 1, i));
                if deg <= 1 {
                    used += 1;
                    remaining &= !(g[i] | 1 << i);
                    updated = true;
                }
            }
        }
        *cur_ma = std::cmp::max(*cur_ma, used);
        // base: there is no vertex remaining
        if remaining == 0 {
            return;
        }
        debug_assert_ne!(degma.0, 0);
        let v = degma.1;
        // We use v or don't use v.
        max_indep_internal(g, used + 1, remaining & !g[v] & !(1 << v), cur_ma);
        max_indep_internal(g, used, remaining & !(1 << v), cur_ma);
    }
    let n = g.len();
    assert!(n <= 64);
    let mut g_bits = vec![0i64; n];
    for i in 0..n {
        assert_eq!(g[i].len(), n);
        for j in 0..n {
            if g[i][j] { g_bits[i] |= 1 << j; }
        }
    }
    let mut ma = 0;
    max_indep_internal(&g_bits, 0, (1 << n) - 1, &mut ma);
    ma
}
