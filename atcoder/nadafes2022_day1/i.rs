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

// Dijkstra's algorithm.
// Verified by: AtCoder ABC164 (https://atcoder.jp/contests/abc164/submissions/12423853)
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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn range<F: FnMut(usize)>(lo: usize, hi: usize, mut f: F) {
    fn inner<F: FnMut(usize)>(lo: usize, hi: usize, k: usize, a: usize, b: usize, f: &mut F) {
        if hi <= a || lo >= b { return; }
        if lo <= a && b <= hi {
            f(k);
            return;
        }
        let mid = (a + b) / 2;
        inner(lo, hi, 2 * k, a, mid, f);
        inner(lo, hi, 2 * k + 1, mid, b, f);
    }
    inner(lo, hi, 1, 0, 1 << 18, &mut f);
}

// https://atcoder.jp/contests/nadafes2022_day1/tasks/nadafes2022_day1_i
// セグメント木と同じ方法で頂点と辺を作り、特定の x 座標・y 座標の範囲に辺を張るという
// 要求に対応する。
fn main() {
    input! {
        n: usize, k: i64,
        xyc: [(i64, i64, i64); n],
    }
    let mut dijk = Dijkstra::new(n + (1 << 20));
    let mut coox = vec![];
    let mut cooy = vec![];
    for &(x, y, _c) in &xyc {
        coox.push(x);
        cooy.push(y);
    }
    coox.sort(); coox.dedup();
    cooy.sort(); cooy.dedup();
    for i in 0..n {
        let (x, y, c) = xyc[i];
        let xi = coox.binary_search(&x).unwrap();
        let yi = cooy.binary_search(&y).unwrap();
        dijk.add_edge(n + (1 << 18) + xi, i, c);
        dijk.add_edge(n + (3 << 18) + yi, i, c);
        // x
        let lo = coox.lower_bound(&(x - k));
        let hi = coox.upper_bound(&(x + k));
        range(lo, hi, |v| dijk.add_edge(i, n + v, c));
        // y
        let lo = cooy.lower_bound(&(y - k));
        let hi = cooy.upper_bound(&(y + k));
        range(lo, hi, |v| dijk.add_edge(i, n + (1 << 19) + v, c));
    }
    // Segtree-like edges
    for i in 1..1 << 18 {
        dijk.add_edge(n + i, n + 2 * i, 0);
        dijk.add_edge(n + i, n + 2 * i + 1, 0);
        dijk.add_edge(n + (1 << 19) + i, n + (1 << 19) + 2 * i, 0);
        dijk.add_edge(n + (1 << 19) + i, n + (1 << 19) + 2 * i + 1, 0);
    }
    const INF: i64 = 1 << 50;
    let sol = dijk.solve(0, INF);
    println!("{}", if sol[n - 1] >= INF { -1 } else { sol[n - 1] });
}
