#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Try to send flow from s to t.
// A single step in Ford-Fulkerson.
// Works with graphs whose edges' capacities are all 1.
// O(V + E)
fn add_flow(g: &mut [Vec<usize>], s: usize, t: usize) -> bool {
    let n = g.len();
    let mut que = VecDeque::new();
    const INF: i32 = 1 << 29;
    let mut dist = vec![INF; n];
    let mut pre = vec![(0, 0); n];
    que.push_back((0, 0, 0, s));
    while let Some((d, p, idx, v)) = que.pop_front() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        pre[v] = (p, idx); // g[p][idx] = v
        for (idx, &w) in g[v].iter().enumerate() {
            que.push_back((d + 1, v, idx, w));
        }
    }
    if dist[t] >= INF {
        return false;
    }
    let mut path = vec![];
    let mut cur = t;
    while dist[cur] > 0 {
        let (p, idx) = pre[cur];
        path.push((p, idx, cur));
        cur = p;
    }
    for &(p, idx, v) in &path {
        assert_eq!(g[p][idx], v);
        assert_eq!(g[p].swap_remove(idx), v);
    }
    // reverse edges
    for &(p, _, v) in &path {
        g[v].push(p);
    }
    true
}

// The author read the editorial before implementing this.
// Tags: flow ford-fulkerson manual-flow
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
        q: usize,
        xy: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; 2 * n + 2];
    for i in 0..n {
        g[2 * n].push(i);
        g[n + i].push(2 * n + 1);
    }
    for &(a, b) in &ab {
        g[a].push(b + n);
    }
    let mut flow = 0;
    loop {
        if add_flow(&mut g, 2 * n, 2 * n + 1) {
            flow += 1;
        } else {
            break;
        }
    }
    for &(x, y) in &xy {
        if let Some(idx) = g[x].iter().position(|&v| v == n + y) {
            // (2-1) We have an edge x -> y and are not using it currently.
            // No hesitation in removing this.
            assert_eq!(g[x].swap_remove(idx), n + y);
        } else {
            if let Some(idx) = g[n + y].iter().position(|&v| v == x) {
                if add_flow(&mut g, x, n + y) {
                    // (2-2) There is a path x -> y in the residual graph;
                    // Remove the reversed edge y -> x.
                    assert_eq!(g[n + y].swap_remove(idx), x);
                } else {
                    // (2-3) There are no paths x -> y in the residual graph;
                    // We have to decrease the flow by 1.
                    // Adding flows x -> s and t -> y solves this.
                    assert!(add_flow(&mut g, x, 2 * n));
                    assert!(add_flow(&mut g, 2 * n + 1, n + y));
                    assert_eq!(g[n + y].swap_remove(idx), x);
                    flow -= 1;
                }
            } else {
                g[x].push(n + y);
                if add_flow(&mut g, 2 * n, 2 * n + 1) {
                    flow += 1;
                }
            }
        }
        puts!("{}\n", ["No", "Yes"][usize::from(flow == n)]);
    }
}
