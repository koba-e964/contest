// Find min x >= 0 s.t. a^x = b
// Verified by: https://atcoder.jp/contests/abc270/submissions/35138862
// Depends on: math/mod_arith.rs
fn bsgs(p: i64, a: i64, b: i64) -> i64 {
    assert!(a >= 0 && a < p);
    assert!(b >= 0 && b < p);
    if a == 0 {
        return if b == 1 { 0 } else if b == 0 { 1 } else { -1 };
    }
    const B: i64 = 32000;
    let ap = powmod(a, B, p);
    let mut hm = std::collections::HashMap::new();
    let mut cur = 1;
    for i in 0..B {
        if !hm.contains_key(&cur) {
            hm.insert(cur, i * B);
        }
        cur *= ap;
        cur %= p;
    }
    let mut mi = p;
    let ainv = powmod(a, p - 2, p);
    cur = b;
    for i in 0..B {
        if let Some(val) = hm.get(&cur) {
            mi = std::cmp::min(mi, val + i);
        }
        cur *= ainv;
        cur %= p;
    }
    let res = if mi >= p {
        -1
    } else {
        mi
    };
     res
}
