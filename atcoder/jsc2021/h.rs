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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

/// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
/// Many-rooted version.
/// Verified by: https://yukicoder.me/submissions/413634
pub struct LowestCommonAncestor {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LowestCommonAncestor {
    fn dfs(&mut self, edges: &[Vec<usize>], v: usize, par: usize, d: usize) {
        self.parent[v] = par;
        self.dep[v] = d;
        for &u in edges[v].iter() {
            if u != par {
                self.dfs(edges, u, v, d + 1);
            }
        }
    }
    fn lca_init(&mut self) {
        let n = self.n;
        for v in 0 .. n {
            self.lca_tbl[v] = vec![0; self.bn + 1];
            self.lca_tbl[v][0] = self.parent[v];
        }
        for i in 1 .. self.bn + 1 {
            for v in 0 .. n {
                self.lca_tbl[v][i] =
                    self.lca_tbl[self.lca_tbl[v][i - 1]][i - 1];
            }
        }
    }
    pub fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let dx = self.dep[x];
        let mut dy = self.dep[y];
        if dx > dy {
            return self.lca(y, x);
        }
        for l in (0 .. self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0 .. self.bn + 1).rev() {
            if self.lca_tbl[x][l] != self.lca_tbl[y][l] {
	        x = self.lca_tbl[x][l];
	        y = self.lca_tbl[y][l];
            }
        }
        self.lca_tbl[x][0]
    }
    #[allow(unused)]
    pub fn depth(&self, a: usize) -> usize {
        self.dep[a]
    }
    #[allow(unused)]
    pub fn parent(&self, a: usize) -> usize {
        self.parent[a]
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LowestCommonAncestor {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
}

struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm: HashMap<i32, i32> = HashMap::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u64 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        x ^ x << 10
    }
}

fn dfs1(v: usize, par: usize, g: &[Vec<usize>], r: usize, root: &mut [usize]) {
    root[v] = r;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        dfs1(w, v, g, r, root);
    }
}

fn dfs2(v: usize, par: usize, g: &[Vec<(usize, i64)>], term: &[i64])
        -> (i64, i64) {
    let mut sum = 0;
    let mut imos = term[v];
    for &(w, c) in &g[v] {
        if w == par {
            continue;
        }
        let (subsum, subimos) = dfs2(w, v, g, term);
        sum += subsum;
        imos += subimos;
        if subimos > 0 {
            sum += c;
        }
    }
    (sum, imos)
}

fn get_root_seq(roots: &[usize], cyc: &[Vec<(usize, i64)>]) -> Vec<(usize, i64)> {
    let mut root_seq = vec![];
    let v = roots[0];
    if roots.len() == 2 {
        for i in 0..2 {
            root_seq.push((roots[i], cyc[v][i].1));
        }
        return root_seq;
    }
    let (mut w, c) = cyc[v][0];
    root_seq.push((v, c));
    while w != v {
        let mut nxt = (0, 0);
        for &(a, b) in &cyc[w] {
            if a == root_seq[root_seq.len() - 1].0 {
                continue;
            }
            nxt = (a, b);
            break;
        }
            root_seq.push((w, nxt.1));
        w = nxt.0;
    }
    root_seq
}

// Tags: namori-graph, almost-tree
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize,
        ac: [(usize1, i64); n],
        xy: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    let mut deg = vec![0; n];
    for i in 0..n {
        let (a, c) = ac[i];
        g[a].push((i, c));
        g[i].push((a, c));
        deg[a] += 1;
        deg[i] += 1;
    }
    let mut que = vec![];
    for i in 0..n {
        if deg[i] == 1 {
            que.push(i);
        }
    }
    let mut rem = vec![true; n];
    while let Some(v) = que.pop() {
        if !rem[v] {
            continue;
        }
        rem[v] = false;
        for &(w, _) in &g[v] {
            if rem[w] {
                deg[w] -= 1;
                if deg[w] == 1 {
                    que.push(w);
                }
            }
        }
    }
    let mut forest = vec![vec![]; n];
    let mut forest_e = vec![vec![]; n];
    let mut cyc = vec![vec![]; n];
    let mut roots = vec![];
    for i in 0..n {
        if rem[i] {
            roots.push(i);
        }
        let (a, c) = ac[i];
        if rem[i] && rem[a] {
            cyc[i].push((a, c));
            cyc[a].push((i, c));
        } else {
            forest[i].push((a, c));
            forest_e[i].push(a);
            forest[a].push((i, c));
            forest_e[a].push(i);
        }
    }
    let mut to_root = vec![0; n];
    for &r in &roots {
        dfs1(r, n, &forest_e, r, &mut to_root);
    }
    let lca = LowestCommonAncestor::new(&forest_e, &roots);
    let root_seq;
    let mut root_inv = vec![n; n];
    {
        root_seq = get_root_seq(&roots, &cyc);
        for i in 0..root_seq.len() {
            root_inv[root_seq[i].0] = i;
        }
    }
    let mut term = vec![0; n];
    let mut qs = vec![];
    for &(x, y) in &xy {
        term[x] += 1;
        term[y] += 1;
        if to_root[x] == to_root[y] {
            let l = lca.lca(x, y);
            term[l] -= 2;
        } else {
            qs.push((root_inv[to_root[x]], root_inv[to_root[y]]));
        }
    }
    let mut sum = 0;
    for &r in &roots {
        let (subsum, _) = dfs2(r, n, &forest, &term);
        sum += subsum;
    }
    // eprintln!("root_seq = {:?}", root_seq);
    // eprintln!("sum = {}, qs = {:?}", sum, qs);
    let r = roots.len();
    let mut val = vec![0u64; qs.len()];
    let mut rng = Rng::new();
    for i in 0..qs.len() {
        val[i] = rng.next();
    }
    let mut imos = vec![0; r];
    for i in 0..qs.len() {
        let (x, y) = qs[i];
        imos[x] ^= val[i];
        imos[y] ^= val[i];
    }
    for i in 0..r - 1 {
        imos[i + 1] ^= imos[i];
    }
    let mut hm = HashMap::new();
    for i in 0..r {
        *hm.entry(imos[i]).or_insert(0) += root_seq[i].1;
        sum += root_seq[i].1;
    }
    let mut ma = 0;
    for (_k, v) in hm {
        ma.chmax(v);
    }
    sum -= ma;
    puts!("{}\n", sum);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
