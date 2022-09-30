use std::cmp::*;
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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// 結婚定理を使うと、S \subseteq [n] について条件 \sum {A cap S != empty} pop[A] >= k * |S| が満たされれば良いことがわかる。これはゼータ変換と算数で計算できる。
// Tags: halls-marriage-theorem, halls-theorem
fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut pass = 2 * n * k;
    let mut fail = n * k - 1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let mut dp = vec![0; 1 << n];
        let mut c = vec![0; mid];
        for i in 0..n {
            for j in 0..(mid + 2 * a[i] - 1) / (2 * a[i]) {
                for l in j * 2 * a[i]..min(j * 2 * a[i] + a[i], mid) {
                    c[l] |= 1 << i;
                }
            }
        }
        for c in c {
            dp[c] += 1;
        }
        for i in 0..n {
            for j in 0..1 << n {
                if (j & 1 << i) != 0 { continue; }
                dp[j | 1 << i] += dp[j];
            }
        }
        if (0..1 << n).all(|i| mid - dp[(1 << n) - 1 - i] >= i.count_ones() as usize * k) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
