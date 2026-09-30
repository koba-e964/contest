fn psieve(n: usize) -> Vec<bool> {
    if n <= 1 {
        return vec![false; n + 1];
    }
    let mut pr = vec![true; n + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..n + 1 {
        if !pr[i] { continue; }
        for j in 2..n / i + 1 {
            pr[i * j] = false;
        }
    }
    pr
}
