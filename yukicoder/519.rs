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

// Tags: weighted-maximum-matching
fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
    }
    let mut dp = vec![0; 1 << n];
    for bits in 1usize..1 << n {
        let bc = bits.count_ones() as usize;
        if bc % 2 != 0 {
            continue;
        }
        if bc > n / 2 + 1 {
            continue;
        }
        let mut me = 0;
        let mut v = 0;
        for i in 0..n {
            if (bits & 1 << i) == 0 {
                continue;
            }
            v = i;
        }
        for j in 0..v {
            if (bits & 1 << j) == 0 {
                continue;
            }
            me = max(me, a[v][j] + dp[bits ^ 1 << v ^ 1 << j]);
        }
        dp[bits] = me;
    }
    let mut ma = 0;
    for bits in 0..1 << n {
        ma = max(ma, dp[bits] + dp[(1 << n) - 1 - bits]);
    }
    println!("{}", ma);
}
