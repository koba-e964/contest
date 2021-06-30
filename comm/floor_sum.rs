// Ported from ACL.
// Computes \sum_{i = 0}^{n - 1} floor((a * i + b) / m).
// Verified by: https://atcoder.jp/contests/arc111/submissions/23877969
fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    fn internal(n: i64, m: i64, mut a: i64, mut b: i64, mut acc: i64) -> i64 {
        if a >= m {
            let q = a / m;
            acc += (n - 1) * n / 2 * q;
            a -= q * m;
        }
        if b >= m {
            let q = b / m;
            acc += n * q;
            b -= q * m;
        }
        let y_max = (a * n + b) / m;
        let x_max = y_max * m - b;
        if y_max == 0 {
            return acc;
        }
        acc += (n - (x_max + a - 1) / a) * y_max;
        let mut sub_b = a - x_max % a;
        if sub_b >= a {
            sub_b -= a;
        }
        internal(y_max, a, m, sub_b, acc)
    }
    internal(n, m, a, b, 0)
}
