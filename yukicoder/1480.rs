use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

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
    let n: usize = get();
    let m: usize = get();
    let mut dijk = Dijkstra::new(n + 2 * m);
    for i in 0..m {
        let k: usize = get();
        let c: i64 = get();
        let s: Vec<usize> = (0..k).map(|_| get()).collect();
        for &s in &s {
            if s % 2 == 0 {
                dijk.add_edge(s - 1, n + 2 * i, s as i64 / 2 + c);
                dijk.add_edge(n + 2 * i, s - 1, s as i64 / 2);
            } else {
                dijk.add_edge(s - 1, n + 2 * i + 1, s as i64 / 2 + c);
                dijk.add_edge(n + 2 * i, s - 1, s as i64 / 2 + 1);
            }
            dijk.add_edge(n + 2 * i + 1, s - 1, s as i64 / 2 + 1);
        }
    }
    const INF: i64 = 1 << 50;
    let sol = dijk.solve(0, INF);
    println!("{}", if sol[n - 1] >= INF { -1 } else { sol[n - 1] });
}
