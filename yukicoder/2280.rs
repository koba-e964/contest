use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let r = a % b;
    let q = a / b;
    let (g, x, y) = ext_gcd(b, r);
    (g, y, x - q * y)
}

fn f(m: i64, a: i64, b: i64, x: i64, y: i64, k: i64) -> i64 {
    assert_eq!(a * x + b * y, 1);
    if k > a { return 0; }
    if k == a {
        if m < a { return 0; }
        let lim = m / a * a;
        return lim / a - 1 - lim / b + lim / (a * b);
    }
    let val1 = (x * k).rem_euclid(b) * a;
    let val2 = (y * k).rem_euclid(a) * b;
    let mut ans = 2 * (m / (a * b));
    let rem = m % (a * b);
    if val1 <= rem {
        ans += 1;
    }
    if val2 <= rem {
        ans += 1;
    }
    ans
}

// https://yukicoder.me/problems/no/2280 (3.5)
// 一般性を失わず gcd(A, B) = 1 としてよい。
// K > A のとき、0 個。
// K = A のとき、A の倍数が B の倍数を挟まずに並ぶところを数えれば良い。
// [0, AB) に A の倍数は B 個、B の倍数は A 個ある。B 個ある A の倍数の間の区間のうち A 個が B の倍数を含むので、それ以外の B-A 個が答え。
// ただし 0 と A の間はカウントしないことに注意。
// K < A のときは A の倍数と B の倍数の差分が K であるところを探せば良い。extgcd でできる。
// -> K = A, M < A のときに -1 にしてしまっていた。修正して AC。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        mabk: [(i64, i64, i64, i64); t],
    }
    for (m, a, b, k) in mabk {
        let (g, x, y) = ext_gcd(a, b);
        puts!("{}\n", if k % g == 0 {
            f(m / g, a / g, b / g, x, y, k / g)
        } else { 0 });
    }
}
