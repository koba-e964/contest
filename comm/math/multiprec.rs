// Decimal
// Verified by: https://atcoder.jp/contests/abc233/submissions/28164785
const RADIX: i32 = 10;

fn mulprec_add(a: &mut [i32], mut x: i32) {
    for a in a.iter_mut().rev() {
        let tmp = *a + x;
        let mut r = tmp % RADIX;
        if r < 0 {
            r += RADIX;
        }
        *a = r;
        x = (tmp - r) / RADIX;
    }
}

fn mulprec_div(a: &mut [i32], x: i32) -> i32 {
    let mut c = 0;
    for a in a {
        c = RADIX * c + *a;
        *a = c / x;
        c %= x;
    }
    c
}
