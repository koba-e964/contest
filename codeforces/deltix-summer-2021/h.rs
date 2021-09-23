use std::cmp::*;
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

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
#[derive(Clone)]
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

const INF: i64 = 1 << 50;

// Ref: https://codeforces.com/blog/entry/69287
// O(krn^2)
fn find_augmenting_path(
    k: usize, g: &[Vec<(usize, i64)>], high_mst: &[(usize, usize)],
    w: &[Vec<i64>], d: &[usize],
    init: &UnionFind, debug: bool,
) -> Vec<(usize, usize)> {
    // We implement this in the simplest setting: no optimizations.
    let n = g.len();
    let mut hg = vec![vec![]; n * n];
    let mut is_left = vec![false; n * n];
    let mut y1 = vec![];
    let mut y2 = vec![];
    let mut e = vec![];
    let mut uf_whole = init.clone();
    // O(n)
    for i in 0..n {
        for &(v, _c) in &g[i] {
            if v < i { continue; }
            assert!(v >= k);
            is_left[i * n + v] = true;
            e.push((i, v));
            assert!(!uf_whole.is_same_set(i, v), "-- uf = {:?}, {} {} {:?}",
                    uf_whole.disj, i, v, g);
            uf_whole.unite(i, v);
        }
    }
    // O(rkn), where r = |e|
    for &(i, v) in &e {
        let mut uf = init.clone();
        for &(j, x) in &e {
            if (j, x) == (i, v) { continue; }
            assert!(is_left[j * n + x]);
            assert!(!uf.is_same_set(j, x), "uf = {:?}, {} {} {:?}", uf.disj, j, x, g);
            uf.unite(j, x);
        }
        for &(l, j) in high_mst {
            if uf.is_same_set(j, l) { continue; }
            if (l, j) == (i, v) { continue; }
            // Only pick edges in the circuit in g + (j, l)
            if !uf_whole.is_same_set(j, l) { continue; }
            assert!(!is_left[l * n + j]);
            // Exchange edge in M1; from E - S to S
            hg[l * n + j].push((i * n + v, w[l][j]));
        }
    }
    // O(kn)
    for &(j, i) in high_mst {
        if uf_whole.is_same_set(i, j) { continue; }
        assert!(!is_left[j * n + i]);
        y1.push((j * n + i, w[i][j]));
    }
    for i in 0..k {
        if g[i].len() < d[i] {
            for j in k..n {
                if !is_left[i * n + j] {
                    y2.push((i * n + j, w[i][j]));
                }
            }
        }
    }
    for &(i, j) in high_mst {
        if i < k { continue; }
        if !is_left[i * n + j] {
            y2.push((i * n + j, w[i][j]));
        }
    }
    // O(kn^2)
    for i in 0..k {
        if d[i] != g[i].len() { continue; }
        for &(a, c) in &g[i] {
            assert!(a >= k);
            for j in k..n {
                if !is_left[i * n + j] {
                    // Exchange edge in M2; from S to E - S
                    hg[i * n + a].push((i * n + j, -c));
                }
            }
        }
    }
    // https://math.mit.edu/~goemans/18438F09/lec11.pdf
    // find the shortest path from y2 to y1
    // hg contains no cycles, so Bellman-Ford is okay
    let mut dist = vec![(INF, 0, 0); n * n];
    for &(v, _) in &y2 {
        dist[v] = (0, 0, v);
    }
    for i in 0..n * n {
        for &(j, _) in &hg[i] {
            assert_ne!(is_left[i], is_left[j]);
        }
    }
    // O(2r * (r + n)kn)
    // The length of any shortest path from Y2 to Y1 is <= 2|S|, because
    // for a path of length 2x exactly x vertices are picked from S and no
    // point is picked twice.
    for _ in 0..2 * e.len() {
        for &(i0, i1) in high_mst {
            let i = i0 * n + i1;
            for &(j, c) in &hg[i] {
                if (dist[j].0, dist[j].1) > (dist[i].0 + c, dist[i].1 + 1) {
                    dist[j] = (dist[i].0 + c, dist[i].1 + 1, i);
                }
            }
        }
    }
    let mut mi = (INF, 0);
    for &(v, c) in &y1 {
        mi = min(mi, (dist[v].0 + c, v));
    }
    if mi.0 >= INF / 2 {
        return vec![];
    }
    // Recover the path from dist
    let mut cur = mi.1;
    let mut path = vec![];
    loop {
        path.push((cur / n, cur % n));
        let pre = dist[cur].2;
        if pre == cur {
            break;
        }
        cur = pre;
    }
    path.reverse();
    if debug {
        eprintln!("y1 = {:?}, y2 = {:?}", y1, y2);
        println!("{:?}", path);
    }
    for j in 0..path.len() {
        let (x, y) = path[j];
        let v = x * n + y;
        assert_eq!(is_left[v], j % 2 != 0);
    }
    path
}

fn calc(uf: &mut UnionFind, d: &[usize], k: usize, w: &[Vec<i64>], debug: bool) -> i64 {
    let n = w.len();
    let mut tot = 0;
    let mut g = vec![vec![]; n];
    if debug {
        for i in 0..k {
            eprintln!("{} => {}", i, uf.root(i));
        }
    }
    let mut e = vec![];
    for i in k..n {
        for j in k..i {
            e.push((w[j][i], j, i));
        }
    }
    e.sort();
    let mut high_mst = vec![];
    {
        let mut uf = UnionFind::new(n);
        for (_, a, b) in e {
            if !uf.is_same_set(a, b) {
                uf.unite(a, b);
                high_mst.push((a, b));
            }
        }
        for i in 0..k {
            for j in k..n {
                high_mst.push((i, j));
            }
        }
    }
    // O(kn^4) in total
    loop {
        if debug {
            let mut ne = 0;
            for i in 0..n {
                ne += g[i].len();
            }
            eprintln!("d = {:?}", d);
            eprintln!("g = {:?}", g);
            eprintln!("|E| = {}", ne / 2);
            eprintln!("tot = {}", tot);
        }
        let path = find_augmenting_path(k, &g, &high_mst, &w, d, &uf, debug);
        if path.is_empty() {
            break;
        }
        let l = path.len() / 2;
        assert_eq!(path.len(), 2 * l + 1);
        for i in 0..l {
            let (x, y) = path[2 * i + 1];
            let gx = g[x].len();
            let gy = g[y].len();
            tot -= w[x][y];
            g[x].retain(|&v| v.0 != y);
            g[y].retain(|&v| v.0 != x);
            assert_ne!(g[x].len(), gx);
            assert_ne!(g[y].len(), gy);
        }
        for i in 0..l + 1 {
            let (x, y) = path[2 * i];
            tot += w[x][y];
            g[x].push((y, w[x][y]));
            g[y].push((x, w[x][y]));
        }
    }
    let mut ne = 0;
    for i in 0..n {
        ne += g[i].len();
    }
    if debug {
        eprintln!("d = {:?}", d);
        eprintln!("g = {:?}", g);
        eprintln!("|E| = {}", ne / 2);
        eprintln!("tot = {}", tot);
        eprintln!();
    }
    let mut nr = 0;
    for i in 0..k {
        if uf.root(i) == i { nr += 1; }
    }
    assert!(ne <= 2 * n - 2 - 2 * (k - nr), "{:?}, ne = {}, nr = {}, uf = {:?}", g, ne, nr, uf.disj);
    if ne == 2 * n - 2 - 2 * (k - nr) {
        tot
    } else {
        INF
    }
}

// The author read the editorial before implementing this.
// Tags: matroid-intersection
fn main() {
    let n: usize = get();
    let k: usize = get();
    let d: Vec<usize> = (0..k).map(|_| get()).collect();
    let mut w = vec![vec![0i64; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let x: i64 = get();
            w[i][j] = x;
            w[j][i] = x;
        }
    }
    let mut pp = vec![];
    for i in 0..k {
        for j in i + 1..k {
            pp.push((i, j));
        }
    }
    let mut mi = INF;
    for bits in 0..1 << (k * (k - 1) / 2) {
        let mut tmp = 0;
        let mut ok = true;
        let mut d = d.clone();
        let mut uf = UnionFind::new(n);
        for i in 0..k * (k - 1) / 2 {
            if (bits & 1 << i) == 0 {
                continue;
            }
            let (p, q) = pp[i];
            if d[p] == 0 || d[q] == 0 || uf.is_same_set(p, q) {
                ok = false;
                break;
            }
            uf.unite(p, q);
            d[p] -= 1;
            d[q] -= 1;
            tmp += w[p][q];
        }
        if !ok {
            continue;
        }
        let sub = calc(&mut uf, &d, k, &w, false);
        mi = min(mi, tmp + sub);
    }
    println!("{}", mi);
}
