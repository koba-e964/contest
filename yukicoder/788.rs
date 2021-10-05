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
        n: usize, m: usize, l: usize1,
        t: [i64; n],
        abc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    const INF: i64 = 1 << 57;
    let mut dist = vec![vec![INF; n]; n];
    let mut que = BinaryHeap::new();
    for i in 0..n {
        que.push((Reverse(0), i));
        while let Some((Reverse(d), v)) = que.pop() {
            if dist[i][v] <= d {
                continue;
            }
            dist[i][v] = d;
            for &(w, c) in &g[v] {
                que.push((Reverse(d + c), w));
            }
        }
    }
    let mut mi = INF;
    for i in 0..n {
        let mut tmp = 0;
        let mut fst = dist[i][l];
        let mut mv = 0;
        for j in 0..n {
            tmp += 2 * t[j] * dist[i][j];
            if i != j {
                mv += t[j];
            }
            if t[j] > 0 {
                fst = min(fst, dist[l][j] - dist[i][j]);
            }
        }
        mi = min(mi, tmp + fst);
        if mv == 0 {
            mi = 0;
        }
    }
    println!("{}", mi);
    
}
