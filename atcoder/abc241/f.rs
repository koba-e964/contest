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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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

fn main() {
    input! {
        _h: i64, _w: i64, n: usize,
        sx: i64, sy: i64,
        gx: i64, gy: i64,
        xy: [(i64, i64); n],
    }
    let mut row = HashMap::new();
    let mut col = HashMap::new();
    let mut st = vec![(sx, sy)];
    for &(x, y) in &xy {
        row.entry(x).or_insert(vec![]).push(y);
        col.entry(y).or_insert(vec![]).push(x);
        st.push((x - 1, y));
        st.push((x + 1, y));
        st.push((x, y - 1));
        st.push((x, y + 1));
    }
    st.sort(); st.dedup();
    for (_, v) in row.iter_mut() {
        v.sort();
    }
    for (_, v) in col.iter_mut() {
        v.sort();
    }
    let gidx = if let Ok(idx) = st.binary_search(&(gx, gy)) {
        idx
    } else {
        println!("-1");
        return;
    };
    let sidx = st.binary_search(&(sx, sy)).unwrap();
    let m = st.len();
    let mut dijk = Dijkstra::new(m);
    for i in 0..m {
        let (x, y) = st[i];
        if let Some(v) = row.get(&x) {
            let a = v.binary_search(&y).unwrap_err();
            if a > 0 {
                let p = st.binary_search(&(x, v[a - 1] + 1)).unwrap();
                dijk.add_edge(i, p, 1);
            }
            if a < v.len() {
                let p = st.binary_search(&(x, v[a] - 1)).unwrap();
                dijk.add_edge(i, p, 1);
            }
        }
        if let Some(v) = col.get(&y) {
            let a = v.binary_search(&x).unwrap_err();
            if a > 0 {
                let p = st.binary_search(&(v[a - 1] + 1, y)).unwrap();
                dijk.add_edge(i, p, 1);
            }
            if a < v.len() {
                let p = st.binary_search(&(v[a] - 1, y)).unwrap();
                dijk.add_edge(i, p, 1);
            }
        }
    }
    const INF: i64 = 1 << 50;
    let dist = dijk.solve(sidx, INF);
    println!("{}", if dist[gidx] >= INF { -1 } else { dist[gidx] });
}
