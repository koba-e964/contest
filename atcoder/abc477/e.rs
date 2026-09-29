fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
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
    // This function returns a Vec consisting of the distances from vertex source.
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
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let b = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut dijk = Dijkstra::new(n + 1);
    for i in 0..n {
        let j = (i + 1) % n;
        dijk.add_edge(i, j, a[i]);
        dijk.add_edge(j, i, a[i]);
        dijk.add_edge(i, n, b[i]);
        dijk.add_edge(n, i, b[i]);
    }
    let dist = dijk.solve(n, 1 << 60);
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        let [s, t] = ints[..] else { panic!() };
        let mut mi = dist[s] + dist[t];
        if t < n {
            mi = mi.min(acc[t] - acc[s]);
            mi = mi.min(acc[n] - acc[t] + acc[s]);
        }
        println!("{mi}");
    }
}
