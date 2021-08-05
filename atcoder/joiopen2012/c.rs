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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, i64)>>, // adjacent list representation
}

impl Dijkstra {
    fn new(n: usize) -> Self {
        Dijkstra { edges: vec![Vec::new(); n] }
    }
    fn add_edge(&mut self, from: usize, to: usize, cost: i64) {
        self.edges[from].push((to, cost));
    }
    /*
     * This function returns a Vec consisting of the distances from vertex source.
     */
    fn solve(&self, source: usize, inf: i64) -> Vec<i64> {
        let n = self.edges.len();
        let mut d = vec![inf; n];
        // que holds (-distance, vertex), so that que.pop() returns the nearest element.
        let mut que = std::collections::BinaryHeap::new();
        que.push((0, source));
        while let Some((cost, pos)) = que.pop() {
            let cost = -cost;
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for &(w, c) in &self.edges[pos] {
                let newcost = cost + c;
                if d[w] > newcost {
                    d[w] = newcost + 1;
                    que.push((-newcost, w));
                }
            }
        }
        return d;
    }
}

const INF: i64 = 1 << 60;
fn calc2(sols: &[Vec<i64>], a: usize, diff: bool, x: i64, y: i64) -> i64 {
    if x == 0 && y == 0 {
        return 0;
    }
    if (x == 0 || y == 0) && !diff {
        if x == 0 {
            let (nx, ny) = if a % 2 == 0 {
                (1, y.abs() - 1)
            } else {
                (0, y.abs())
            };
            let d = min(sols[1][0] + sols[0][3], sols[1][2] + sols[2][3]);
            return d.saturating_mul(nx)
                .saturating_add(sols[1][3].saturating_mul(ny));
        } else {
            let (nx, ny) = if a % 2 == 1 {
                (x.abs() - 1, 1)
            } else {
                (x.abs(), 0)
            };
            let d = min(sols[0][1] + sols[1][2], sols[0][3] + sols[3][2]);
            return sols[0][2].saturating_mul(nx)
                .saturating_add(d.saturating_mul(ny));
        };
    }
    let hor = sols[0][2];
    let ver = sols[1][3];
    let diag = if x * y > 0 {
        sols[2][1].saturating_add(sols[0][3])
    } else {
        sols[2][3].saturating_add(sols[0][1])
    };
    let x = x.abs();
    let y = y.abs();
    if diff {
        let m = min(x, y);
        return hor.saturating_mul(x - m)
            .saturating_add(ver.saturating_mul(y - m))
            .saturating_add(min(diag, hor + ver).saturating_mul(m));
    }
    let m = min(x, y) - 1;
    hor.saturating_mul(x - m - 1)
        .saturating_add(ver.saturating_mul(y - m - 1))
        .saturating_add(diag)
        .saturating_add(min(diag, hor + ver).saturating_mul(m))
}

fn calc(a: usize, b: usize, sols: &[Vec<i64>], x: i64, y: i64) -> i64 {
    let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let diff = (a + b) % 2 != 0;
    let mut mi = sols[a][b].saturating_add(
        min(calc2(sols, b % 2, diff, x, y), calc2(sols, a % 2, diff, x, y)));
    let nx = x - dir[a].0;
    let ny = y - dir[a].1;
    mi.chmin(sols[a ^ 2][b].saturating_add(
        min(calc2(sols, b % 2, diff, nx, ny), calc2(sols, a % 2, diff, nx, ny))));
    let nx = x + dir[b].0;
    let ny = y + dir[b].1;
    mi.chmin(sols[a][b ^ 2].saturating_add(
        min(calc2(sols, b % 2, diff, nx, ny), calc2(sols, a % 2, diff, nx, ny))));
    let nx = x + dir[b].0 - dir[a].0;
    let ny = y + dir[b].1 - dir[a].1;
    mi.chmin(sols[a ^ 2][b ^ 2].saturating_add(
        min(calc2(sols, b % 2, diff, nx, ny), calc2(sols, a % 2, diff, nx, ny))));
    mi
}

// Tags: dijkstra, shortest-path-in-big-graphs, implementation, grid, brute-force
fn main() {
    input! {
        n: usize, m: usize, v: usize1, x: i64, y: i64, w: usize1,
        abt: [(usize1, usize1, i64); m],
    }
    let mut dijk = Dijkstra::new(n);
    for &(a, b, t) in &abt {
        dijk.add_edge(a, b, t);
        dijk.add_edge(b, a, t);
    }
    let mut sols = vec![vec![]; 4];
    for i in 0..4 {
        sols[i] = dijk.solve(i, INF);
    }
    let mut mi = INF;
    let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    for i in 0..4 {
        for j in 0..4 {
            let nx = x - dir[i].0 + dir[j].0;
            let ny = y - dir[i].1 + dir[j].1;
            mi.chmin(sols[i][v]
                     .saturating_add(sols[j][w])
                     .saturating_add(calc(i ^ 2, j ^ 2, &sols, nx, ny)));
        }
    }
    if x == 0 && y == 0 {
        let sol = dijk.solve(v, INF);
        mi.chmin(sol[w]);
    }
    println!("{}", if mi >= INF { -1 } else { mi });
}
