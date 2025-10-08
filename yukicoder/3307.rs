fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// Ported from ACL.
// Computes \sum_{i = 0}^{n - 1} floor((a * i + b) / m).
// Verified by: https://atcoder.jp/contests/arc111/submissions/23877969
fn floor_sum(n: i128, m: i128, a: i128, b: i128) -> i128 {
    fn internal(n: i128, m: i128, mut a: i128, mut b: i128, mut acc: i128) -> i128 {
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

fn calc(a: i64, b: i64, c: i64, d: i64) -> i64 {
    let num = a * d - b * c;
    let den = b * d;
    let lim = (den + num - 1) / num;
    let lim = lim as i128;
    (lim - 1 - floor_sum(lim, 2 * b as i128, 2 * a as i128, b as i128) + floor_sum(lim, 2 * d as i128, 2 * c as i128, d as i128)) as i64
}

// Tags: round, floor-sum
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [a, b, c, d] = ints[..] else { panic!() };
    if a * d == b * c {
        println!("-1");
        return;
    }
    let ans = if a * d > b * c {
        calc(a, b, c, d)
    } else {
        calc(c, d, a, b)
    };
    println!("{ans}");
}
