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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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
        let mut que = std::collections::VecDeque::new();
        que.push_back(State {cost: 0, position: source});
        while let Some(State {cost, position: pos}) = que.pop_front() {
            if d[pos] <= cost {
                continue;
            }
            d[pos] = cost;
            for adj in &self.edges[pos] {
                que.push_back(State {cost: cost + adj.1, position: adj.0});
            }
        }
        return d;
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        m: usize, r: usize,
    }
    let mut board = vec![vec![0; 3]; 4];
    for i in 0..3 {
        for j in 0..3 {
            board[i][j] = 3 * (2 - i) + j + 1;
        }
    }
    board[3][0] = 0;
    board[3][1] = 11;
    board[3][2] = 11;
    let mut dist = vec![vec![100; 10]; 10];
    for i in 0..10 {
        dist[i][i] = 0;
    }
    for i in 0..4 {
        for j in 0..3 {
            if i < 3 {
                let a = board[i][j];
                let b = board[i + 1][j];
                if a < 10 && b < 10 {
                    dist[a][b] = 1;
                    dist[b][a] = 1;
                }
            }
            if j < 2 {
                let a = board[i][j];
                let b = board[i][j + 1];
                if a < 10 && b < 10 {
                    dist[a][b] = 1;
                    dist[b][a] = 1;
                }
            }
        }
    }
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut dijk = Dijkstra::new(10 * m);
    const INF: i64 = 1 << 50;
    for i in 0..m {
        for j in 0..10 {
            let to = (10 * i + j) % m;
            dijk.add_edge(j * m + i, j * m + to, 1);
            for k in 0..10 {
                if dist[j][k] == 1 {
                    dijk.add_edge(j * m + i, k * m + i, 1);
                }
            }
        }
    }
    let sol = dijk.solve(0, INF);
    let mut mi = INF;
    for i in 0..10 {
        mi = min(mi, sol[i * m + r]);
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
