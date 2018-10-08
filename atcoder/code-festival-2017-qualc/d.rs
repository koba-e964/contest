#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        s: chars,
    }
    let s: Vec<i32> = s.into_iter()
        .map(|c| c as u8 as i32 - 'a' as u8 as i32).collect();
    let n = s.len();
    let mut pre = vec![INF; 1 << 26];
    const INF: i32 = 1 << 28;
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    pre[0] = 0;
    let mut acc = 0;
    for i in 0 .. n {
        let idx = s[i];
        acc ^= 1 << idx;
        if pre[acc] < INF {
            dp[i + 1] = min(dp[i + 1], dp[pre[acc] as usize] + 1);
        }
        if acc == 0 {
            dp[i + 1] = 1;
        }
        pre[acc] = i as i32 + 1;
        for j in 0 .. 26 {
            let nxt = acc ^ 1 << j;
            if pre[nxt] < INF {
                if dp[i + 1] > dp[pre[nxt] as usize] + 1 {
                }
                dp[i + 1] = min(dp[i + 1], dp[pre[nxt] as usize] + 1);
            }
            if nxt == 0 {
                dp[i + 1] = 1;
            }
        }
    }
    puts!("{}\n", dp[n]);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
