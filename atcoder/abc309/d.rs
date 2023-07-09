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
        n1: usize, n2: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n1 + n2];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    const INF: i32 = 1 << 28;
    let mut dist = vec![INF; n1 + n2];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    que.push_back((0, n1 + n2 - 1));
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &w in &g[v] {
            que.push_back((d + 1, w));
        }
    }
    let ma1 = dist[..n1].iter().max().unwrap();
    let ma2 = dist[n1..].iter().max().unwrap();
    println!("{}", ma1 + ma2 + 1);
}
