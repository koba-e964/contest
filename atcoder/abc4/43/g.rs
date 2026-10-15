fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// Ported from ACL.
// Computes \sum_{i = 0}^{n - 1} floor((a * i + b) / m).
// Verified by: https://atcoder.jp/contests/arc111/submissions/23877969
fn floor_sum(n: i64, m: i64, a: i64, b: i64) -> i64 {
    fn internal(n: i64, m: i64, mut a: i64, mut b: i64, mut acc: i64) -> i64 {
        if a < 0 {
            let q = (-a - 1) / m + 1;
            acc -= (n - 1) * n / 2 * q;
            a += q * m;
        }
        if a >= m {
            let q = a / m;
            acc += (n - 1) * n / 2 * q;
            a -= q * m;
        }
        if b < 0 {
            let q = (-b - 1) / m + 1;
            acc -= n * q;
            b += q * m;
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

// https://atcoder.jp/contests/abc443/tasks/abc443_g
// A mod M < B mod M <=> (A-B) mod M + B mod M = A mod M + M
// よって、 [A mod M < B mod M] = ((A-B) mod M + B mod M - A mod M) / M
// = floor(A/M) - floor(B/M) - floor((A-B)/M)
fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [n, m, a, b] = ints[..] else { panic!() };
        let ans = floor_sum(n, m, 1, 0);
        let ans = ans - floor_sum(n, m, a, b);
        let ans = ans - floor_sum(n, m, 1-a, -b);
        println!("{ans}");
    }
}
