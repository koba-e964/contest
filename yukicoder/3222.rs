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
        n: usize, m: usize,
        p: [i64; n],
        ab: [(usize1, usize1); n - 1],
        c: [usize1; m],
    }
    let mut deg = vec![0; n];
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
        deg[a] += 1;
        deg[b] += 1;
    }
    let mut que = VecDeque::new();
    let mut dist = vec![std::usize::MAX; n];
    for &x in &c {
        que.push_back((0, x));
    }
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        for &u in &g[v] {
            que.push_back((d + 1, u));
        }
    }
    let mut que = BinaryHeap::new();
    for i in 0..n {
        if deg[i] == 1 {
            que.push((p[i], i));
        }
    }
    let mut ans = 0;
    let mut t = 0;
    loop {
        if que.is_empty() { break; }
        let (x, v) = que.pop().unwrap();
        if dist[v] <= t { continue; }
        ans += x;
        t += 1;
        for &u in &g[v] {
            deg[u] -= 1;
            if deg[u] == 1 {
                que.push((p[u], u));
            }
        }
    }
    println!("{ans}");
}
