/// parameters: W: size
/// outputs: pr: Vec<bool> and pd: Vec<Vec<usize>>
/// Verified by: http://codeforces.com/contest/547/submission/69474590
{
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut pd = vec![vec![]; W];
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 1..(W - 1) / i + 1 {
            pd[i * j].push(i);
        }
    }
}
