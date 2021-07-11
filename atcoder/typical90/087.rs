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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn calc(a: &[Vec<i64>], x: i64, p: i64) -> usize {
    let mut dist = a.to_vec();
    let n = a.len();
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == -1 {
                dist[i][j] = x;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        for j in 0..i {
            if dist[i][j] <= p {
                ans += 1;
            }
        }
    }
    ans
}

fn main() {
    input! {
        n: usize, p: i64, k: usize,
        a: [[i64; n]; n],
    }
    let inf = 1i64 << 40;
    let sub = calc(&a, inf, p);
    if sub == k {
        println!("Infinity");
        return;
    }
    if sub > k {
        println!("0");
        return;
    }
    let mut pass = 0;
    let mut fail = inf;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if calc(&a, mid, p) >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    if pass == 0 {
        println!("0");
        return;
    }
    let r = pass;
    pass = 0;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        if calc(&a, mid, p) > k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let l = pass;
    println!("{}", r - l);
}
