// Depends on: MInt.rs
// Verified by: https://yukicoder.me/submissions/853225
const B: usize = 2;

fn squmul(a: &[[MInt; B]], b: &[[MInt; B]]) -> [[MInt; B]; B] {
    let mut ret = [[MInt::new(0); B]; B];
    for i in 0..B {
        for j in 0..B {
            for k in 0..B {
                ret[i][k] += a[i][j] * b[j][k];
            }
        }
    }
    ret
}

fn squpow(a: &[[MInt; B]; B], mut e: i64) -> [[MInt; B]; B] {
    let mut sum = [[MInt::new(0); B]; B];
    for i in 0..B { sum[i][i] = 1.into(); }
    let mut cur = *a;
    while e > 0 {
        if e % 2 == 1 {
            sum = squmul(&sum, &cur);
        }
        cur = squmul(&cur, &cur);
        e /= 2;
    }
    sum
}
