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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        uva: [(usize1, usize1, i32); m],
        s: [usize1; k],
    }
    let mut g = vec![vec![]; 2 * n];
    for (u, v, a) in uva {
        if a == 1 {
            g[u].push((v, 1));
            g[v].push((u, 1)); 
        } else {
            g[u + n].push((v + n, 1));
            g[v + n].push((u + n, 1));
        }
    }
    for s in s {
        g[s].push((s + n, 0));
        g[s + n].push((s, 0));
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; 2 * n];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &(w, c) in &g[v] {
            if c == 0 {
                que.push_front((d, w));
            } else {
                que.push_back((d + 1, w));
            }
        }
    }
    let x = min(dist[n - 1], dist[2 * n - 1]);
    println!("{}", if x >= INF { -1 } else { x });
}
