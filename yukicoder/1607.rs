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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        uvc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, c) in &uvc {
        g[u].push((v, c));
        g[v].push((u, c));
    }
    let mut fail = 200_100;
    let mut pass = 0;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut que = VecDeque::new();
        const INF: usize = 1 << 20;
        let mut dist = vec![INF; n];
        que.push_back((0, 0));
        while let Some((d, v)) = que.pop_front() {
            if dist[v] <= d {
                continue;
            }
            dist[v] = d;
            for &(w, c) in &g[v] {
                if c >= mid {
                    que.push_back((d + 1, w));
                } else {
                    que.push_front((d, w));
                }
            }
        }
        if dist[n - 1] >= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
