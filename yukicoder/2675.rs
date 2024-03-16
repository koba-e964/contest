use std::cmp::*;
use std::collections::*;
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

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut map = HashMap::new();
    for i in 0..n {
        map.insert(xy[i], i);
    }
    let mut poss = HashSet::new();
    for &(x, y) in &xy {
        for &d in &dir {
            for idx in 0..2 {
                let nx = x + 2 * d.0 + (2 * idx - 1) * d.1;
                let ny = y + 2 * d.1 - (2 * idx - 1) * d.0;
                if !map.contains_key(&(nx, ny)) {
                    poss.insert((nx, ny));
                }
            }
        }
    }
    const INF: i32 = 1 << 29;
    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for (x, y) in poss {
        let mut ep = dp.clone();
        for &d in &dir {
            let mut now = 0;
            for idx in 0..2 {
                let nx = x + 2 * d.0 + (2 * idx - 1) * d.1;
                let ny = y + 2 * d.1 - (2 * idx - 1) * d.0;
                if let Some(&idx) = map.get(&(nx, ny)) {
                    now |= 1 << idx;
                }
            }
            for bits in 0..1 << n {
                ep[bits | now] = min(ep[bits | now], dp[bits] + 1);
            }
        }
        dp = ep;
    }
    let ans = dp[(1 << n) - 1];
    println!("{}", if ans >= INF { -1 } else { ans });
}
