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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        fs: [(usize, i64); n],
    }
    let mut v = vec![vec![]; n];
    for (f, s) in fs {
        v[f - 1].push(s);
    }
    let mut top = vec![];
    for i in 0..n {
        v[i].sort(); v[i].reverse();
        if !v[i].is_empty() {
            top.push(v[i][0]);
        }
    }
    top.sort(); top.reverse();
    let mut ans = 0;
    if top.len() >= 2 {
        ans = top[0] + top[1];
    }
    for i in 0..n {
        if v[i].len() >= 2 {
            ans = max(ans, v[i][0] + v[i][1] / 2);
        }
    }
    println!("{}", ans);
}
