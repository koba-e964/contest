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

// Tags: inversion-count, bit-dp
fn main() {
    input! {
        n: usize, x: i64, y: i64,
        a: [i64; n],
        b: [i64; n],
    }
    const INF: i64 = 1 << 60;
    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for bits in 1usize..1 << n {
        let bc = bits.count_ones() as usize;
        let mut mi = INF;
        for i in 0..n {
            if (bits & 1 << i) == 0 { continue; }
            let mut tmp = dp[bits ^ 1 << i] + (b[bc - 1] - a[i]).abs() * x;
            for j in i + 1..n {
                if (bits & 1 << j) != 0 {
                    tmp += y;
                }
            }
            mi = min(mi, tmp);
        }
        dp[bits] = mi;
    }
    println!("{}", dp[(1 << n) - 1]);
}
