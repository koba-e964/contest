#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn calc(n: i64) -> i64 {
    const B: usize = 61;
    // popcount(x + n) - popcount(x) + B
    let mut dp = vec![vec![vec![false; 2]; 2 * B + 1]; B + 1];
    let mut pre = vec![vec![vec![vec![vec![]; 2]; 2]; 2 * B + 1]; B + 1];
    dp[0][B][0] = true;
    for i in 0..B {
        for j in 0..2 * B + 1 {
            for carry in 0..2 {
                if !dp[i][j][carry] { continue; }
                let val = (n >> i) + carry as i64;
                // 0
                dp[i + 1][j + (val & 1) as usize][(carry as i64 & (n >> i)) as usize] = true;
                pre[i + 1][j + (val & 1) as usize][(carry as i64 & (n >> i)) as usize][0].push((j, carry));
                // 1
                if val % 2 == 1 && j >= 1 {
                    dp[i + 1][j - 1][(((val + 1) >> 1) - (n >> (i + 1))) as usize] = true;
                    pre[i + 1][j - 1][(((val + 1) >> 1) - (n >> (i + 1))) as usize][1].push((j, carry));
                }
            }
        }
        if i < 3 {
            // eprintln!("dp[{}] = {:?}", i + 1, &dp[i + 1][B - 5..B + 5]);
        }
    }
    assert!(dp[B][B][0]);
    let mut ans = 0;
    let mut delta = B;
    let mut carry = 0;
    for i in (0..B).rev() {
        if !pre[i + 1][delta][carry][0].is_empty() {
            pre[i + 1][delta][carry][0].sort_by_key(|&(_, c)| c);
            let (nd, nc) = pre[i + 1][delta][carry][0][0];
            delta = nd;
            carry = nc;
            continue;
        }
        pre[i + 1][delta][carry][1].sort_by_key(|&(_, c)| c);
        let (nd, nc) = pre[i + 1][delta][carry][1][0];
        delta = nd;
        carry = nc;
        ans |= 1 << i;
    }
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input!(n: [i64]);
    for n in n {
        puts!("{}\n", calc(n));
    }
}

fn main() {
    solve();
}
