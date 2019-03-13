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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/*
 * Dijkstra's algorithm.
 * Verified by: AtCoder ABC035 (http://abc035.contest.atcoder.jp/submissions/676539)
 */

struct Dijkstra {
    edges: Vec<Vec<(usize, i64)>>, // adjacent list representation
}

/*
 * Code from https://doc.rust-lang.org/std/collections/binary_heap/
 */
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering here
        match other.cost.cmp(&self.cost) {
            std::cmp::Ordering::Equal => other.position.cmp(&self.position),
            x => x,
        }
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
        let mut que = std::collections::BinaryHeap::new();
        que.push(State {cost: 0, position: source});
        while let Some(State {cost, position: pos}) = que.pop() {
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for adj in &self.edges[pos] {
                que.push(State {cost: cost + adj.1, position: adj.0});
            }
        }
        return d;
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize,
        w: usize,
        a: i64,
        b: i64,
        c: [chars; h],
    }
    let mut dijk = Dijkstra::new(2 * h * w);
    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    let tdx = [1, 1, -1, -1];
    let tdy = [-1, 1, -1, 1];
    for i in 0..h {
        for j in 0..w {
            for d in 0..4 { 
                let nx = i as i32 + dx[d];
                let ny = j as i32 + dy[d];
                if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if c[nx][ny] == '#' {
                } else {
                    dijk.add_edge(i * w + j, nx * w + ny, a);
                }
                dijk.add_edge(h * w + i * w + j, nx * w + ny, a);
            }
            for d in 0..4 { 
                let nx = i as i32 + tdx[d];
                let ny = j as i32 + tdy[d];
                if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                dijk.add_edge(h * w + i * w + j, nx * w + ny, 2 * a);
            }
            let mut nearbomb = false;
            for dx in -1..2 {
                for dy in -1..2 {
                    let nx = i as i32 + dx;
                    let ny = j as i32 + dy;
                    if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if c[nx][ny] == '*' {
                        nearbomb = true;
                        break;
                    }
                }
            }
            if !nearbomb {
                dijk.add_edge(i * w + j, h * w + i * w + j, b);
            }
        }
    }
    let mut s = 0;
    let mut g = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                s = i * w + j;
            }
            if c[i][j] == 'g' {
                g = i * w + j;
            }
        }
    }
    const INF: i64 = 1 << 58;
    let d = dijk.solve(s, INF)[g];
    //let d = min(d, dijk.solve(s, INF)[h * w + g]);
    if d == INF {
        puts!("INF\n");
    } else {
        puts!("{}\n", d);
    }
}

fn main() {
    solve();
}
