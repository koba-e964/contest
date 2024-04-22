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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn rec(m: i32, cs: &[(f64, Vec<i32>)], memo: &mut HashMap<i32, f64>) -> f64 {
    if m == 0 {
        return 0.0;
    }
    if let Some(&res) = memo.get(&m) {
        return res;
    }
    let mut res = 1.0f64 / 0.0;
    for &(c, ref s) in cs {
        let mut t = 0.0;
        let mut nonzero = 0.0;
        for j in 0..s.len() {
            if s[j] == 0 {
                continue;
            }
            nonzero += 1.0;
            t += rec((m - s[j]).max(0), cs, memo);
        }
        res = res.min((t + c * s.len() as f64) / nonzero);
    }
    memo.insert(m, res);
    res
}

fn solve() {
    input! {
        n: i32, m: i32,
        cs: [(f64, [i32]); n],
    }
    let mut memo = HashMap::new();
    println!("{}", rec(m, &cs, &mut memo));
}
