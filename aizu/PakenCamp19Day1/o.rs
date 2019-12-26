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
                if adj.1 == 0 {
                    que.push_front(State {cost: cost + adj.1, position: adj.0});
                } else {
                    que.push_back(State {cost: cost + adj.1, position: adj.0});
                }
            }
        }
        return d;
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize, k: usize,
        a: [chars; h],
    }
    let mut dijk = Dijkstra::new(h * w);
    let dxy = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut st = 0;
    let mut en = 0;
    let mut apples = vec![];
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            match a[i][j] {
                'a' => apples.push(v),
                's' => st = v,
                'e' => en = v,
                _ => (),
            };
            if a[i][j] == '#' { continue; }
            for d in 0..4 {
                let (dx, dy) = dxy[d];
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if a[nx][ny] == '#' { continue; }
                dijk.add_edge(v, nx * w + ny, 1);
            }
        }
    }
    let m = apples.len();
    const INF: i64 = 1 << 20;
    let mut sdist = vec![0; m];
    {
        let tmp = dijk.solve(st, INF);
        for i in 0..m {
            sdist[i] = tmp[apples[i]];
        }
    }
    let mut tdist = vec![0; m];
    {
        let tmp = dijk.solve(en, INF);
        for i in 0..m {
            tdist[i] = tmp[apples[i]];
        }
    }
    let mut mat = [[0; 20]; 20];
    for i in 0..m {
        let tmp = dijk.solve(apples[i], INF);
        for j in 0..m {
            mat[i][j] = tmp[apples[j]] as i32;
        }
    }
    let mut dp = vec![[INF as i32; 20]; 1 << m];
    for i in 0..m {
        dp[1 << i][i] = sdist[i] as i32;
    }
    for bits in 0usize..1 << m {
        for j in 0..m {
            if (bits & 1 << j) == 0 { continue; }
            for i in 0..m {
                if (bits & 1 << i) != 0 { continue; }
                let to = bits ^ 1 << i;
                dp[to][i] = min(dp[to][i], dp[bits][j] + mat[j][i]);
            }
        }
    }
    let mut mi = INF as i32;
    for bits in 0usize..1 << m {
        if bits.count_ones() < k as u32 { continue; }
        for i in 0..m {
            if (bits & 1 << i) == 0 { continue; }
            mi = min(mi, dp[bits][i] + tdist[i] as i32);
        }
    }
    puts!("{}\n", if mi >= INF as i32 { -1 } else { mi });
}
