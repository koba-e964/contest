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

fn sq((x, y): (i64, i64), (z, w): (i64, i64)) -> i64 {
    (x - z) * (x - z) + (y - w) * (y - w)
}

fn main() {
    input! {
        n: usize,
        s: (i64, i64), t: (i64, i64),
        xyr: [((i64, i64), i64); n],
    }
    let mut g = vec![vec![false; n]; n];
    for i in 0..n {
        for j in 0..n {
            let (c1, r1) = xyr[i];
            let (c2, r2) = xyr[j];
            let sq = sq(c1, c2);
            if sq >= (r1 - r2) * (r1 - r2) && sq <= (r1 + r2) * (r1 + r2) {
                g[i][j] = true;
            }
        }
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        let (c, r) = xyr[i];
        if sq(c, s) == r * r {
            que.push_back(i);
        }
    }
    let mut vis = vec![false; n];
    while let Some(v) = que.pop_front() {
        if vis[v] { continue; }
        vis[v] = true;
        for w in 0..n {
            if g[v][w] {
                que.push_back(w);
            }
        }
    }
    for i in 0..n {
        let (c, r) = xyr[i];
        if sq(c, t) == r * r && vis[i] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
