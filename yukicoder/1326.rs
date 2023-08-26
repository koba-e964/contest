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

// Lowest Common Ancestor. Call lca(x, y) to get the lca of them.
// Many-rooted version.
// Verified by: https://yukicoder.me/submissions/714482
// These functions use O(n) stack space. 
pub struct LCA {
    n: usize,
    bn: usize,
    parent: Vec<usize>, // r is root <=> parent[r] = r
    dep: Vec<usize>,
    lca_tbl: Vec<Vec<usize>>
}

impl LCA {
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
        for i in 1..self.bn + 1 {
            for v in 0..n {
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
        for l in (0..self.bn + 1).rev() {
            if dy - dx >= 1 << l {
                y = self.lca_tbl[y][l];
                dy -= 1 << l;
            }
        }
        assert_eq!(dx, dy);
        
        if x == y {
            return x;
        }
        for l in (0..self.bn + 1).rev() {
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
    #[allow(unused)]
    pub fn ancestor(&self, v: usize, dist: usize) -> usize {
        let mut ans = v;
        for i in 0..self.bn + 1 {
            if (dist & 1 << i) != 0 {
                ans = self.lca_tbl[ans][i];
            }
        }
        ans
    }
    #[allow(unused)]
    pub fn child(&self, anc: usize, des: usize) -> usize {
        let x = self.dep[des] - self.dep[anc] - 1;
        let mut ans = des;
        for i in 0..self.bn + 1 {
            if (x & 1 << i) != 0 {
                ans = self.lca_tbl[ans][i];
            }
        }
        ans
    }
    pub fn new(edges: &[Vec<usize>], roots: &[usize]) -> Self {
        let n = edges.len();
        let bn = (n.next_power_of_two() - 1).count_ones() as usize;
        let mut ret = LCA {
            n: n, bn: bn, parent: vec![0; n], dep: vec![0; n],
            lca_tbl: vec![Vec::new(); n] };
        for &r in roots {
            ret.dfs(edges, r, r, 0);
        }
        ret.lca_init();
        ret
    }
}

// Ported from https://kokiymgch.hatenablog.com/entry/2018/03/21/174958
// This library uses O(n) stack space. 

//tree index : [0, 1, ..., bc.size() - 1] -> component
//tree index : [bc.size(), bc.size() + 1, ..., bc.size() + art.size() - 1] -> articulation point
//cmp[index of edge] -> index of the node of the constructed tree
//cmp_node[index of node] -> -1 if it's not an articulation point, otherwise index of the node of the constructed tree
pub struct Bicomp {
    pub cmp: Vec<Option<usize>>,
    pub cmp_node: Vec<Option<usize>>,
    pub g: Vec<Vec<usize>>,
    pub e2i: std::collections::HashMap<(usize, usize), usize>,
    pub tree: Vec<Vec<usize>>,
}

impl Bicomp {
    fn dfs(
        u: usize, prev: usize,
        used: &mut [bool],
        ord: &mut [usize],
        low: &mut [usize],
        k: &mut usize,
        tmp: &mut Vec<(usize, usize)>,
        bc: &mut Vec<Vec<(usize, usize)>>,
        art: &mut Vec<usize>,
        g: &[Vec<usize>],
    ) {
        let n = g.len();
        used[u] = true;
        ord[u] = *k;
        *k += 1;
        low[u] = ord[u];
        let mut is_art = false;
        let mut cnt = 0;
        for &v in &g[u] {
            if v != prev {
                if ord[v] >= n || ord[v] < ord[u] {
                    tmp.push((std::cmp::min(u, v), std::cmp::max(u, v)))
                }
                if !used[v] {
                    cnt += 1;
                    Self::dfs(v, u, used, ord, low, k, tmp, bc, art, g);
                    low[u] = std::cmp::min(low[u], low[v]);
                    if prev < n && low[v] >= ord[u] {
                        is_art = true;
                    }
                    if low[v] >= ord[u] {
                        bc.push(vec![]);
                        loop {
                            let e = tmp.pop().unwrap();
                            let idx = bc.len() - 1;
                            bc[idx].push(e);
                            if (std::cmp::min(u, v), std::cmp::max(u, v)) == e {
                                break;
                            }
                        }
                    }
                } else {
                    low[u] = std::cmp::min(low[u], ord[v]);
                }
            }
        }
        if prev >= n && cnt > 1 {
            is_art = true;
        }
        if is_art {
            art.push(u);
        }
    }
    pub fn new(n: usize, edges: &[(usize, usize)]) -> Self {
        let m = edges.len();
        let mut g = vec![vec![]; n];
        let mut e2i = std::collections::HashMap::new();
        for i in 0..m {
            let (u, v) = edges[i];
            g[u].push(v);
            g[v].push(u);
            e2i.insert((std::cmp::min(u, v), std::cmp::max(u, v)), i);
        }
        let mut cmp_node = vec![None; n];
        let mut cmp = vec![None; m];
        let mut ord = vec![n; n];
        let mut low = vec![n; n];
        let mut used = vec![false; n];
        let mut bc = vec![];
        let mut art = vec![];
        let mut tmp = vec![];
        let mut k = 0;

        // build() in original
        Self::dfs(0, n, &mut used, &mut ord, &mut low, &mut k, &mut tmp, &mut bc, &mut art, &g);
        for i in 0..bc.len() {
            for &e in &bc[i] {
                let idx = e2i[&(std::cmp::min(e.0, e.1), std::cmp::max(e.1, e.0))];
                cmp[idx] = Some(i);
            }
        }
        let mut tree = vec![vec![]; bc.len() + art.len()];
        for i in 0..art.len() {
            let j = i + bc.len();
            cmp_node[art[i]] = Some(j);
            let u = art[i];
            let mut tmp = std::collections::HashSet::new();
            for &v in &g[u] {
                let t = cmp[e2i[&(std::cmp::min(u, v), std::cmp::max(v, u))]].unwrap();
                tmp.insert(t);
            }
            for v in tmp {
                tree[j].push(v);
                tree[v].push(j);
            }
        }
        Bicomp {
            cmp: cmp,
            cmp_node: cmp_node,
            g: g,
            e2i: e2i,
            tree: tree,
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], d: i64, dep: &mut [i64]) {
    dep[v] = d;
    for &w in &g[v] {
        if w == par { continue; }
        dfs(w, v, g, d + 1, dep);
    }
}

// https://yukicoder.me/problems/no/1326 (4)
// グラフが木であれば、(x と y の間にある頂点数) = (x と y の距離) - 1 なので簡単。
// 一般のグラフの場合、二重辺連結成分分解を行ってできた木の各頂点について、
// 元のグラフにおける寄与が 0, 1, 2 のいずれかである。
// これは、頂点数 1 の成分については木の頂点をそのままにし、頂点数が 2 以上である成分については
// 木における頂点を通るだけで距離が 1 増えるように、木の頂点を増幅させれば良い。
// それは元々の頂点を v としたとき v に接続する辺ごとに新しく頂点を作りそれぞれの辺と接続させ、
// それらと v を距離 0.5 で繋げばできる。
// Similar problems: https://yukicoder.me/problems/no/1983
// -> 間違い。二重辺連結成分分解ではなく二重頂点連結成分分解をする必要がある。
// これは関節点と結びつく概念である。他人のライブラリを拝借して AC。
// Tags: articulation-points, biconnected-components
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        uv: [(usize1, usize1); m],
        q: usize,
        xy: [(usize1, usize1); q],
    }
    let bct = Bicomp::new(n, &uv);
    let lca = LCA::new(&bct.tree, &[0]);
    let mut dep = vec![0; bct.tree.len()];
    dfs(0, bct.tree.len(), &bct.tree, 0, &mut dep);
    for (x, y) in xy {
        if x == y {
            puts!("0\n");
            continue;
        }
        let bx = if let Some(bx) = bct.cmp_node[x] {
            bx
        } else {
            let w = bct.g[x][0];
            let e = bct.e2i[&(std::cmp::min(x, w), std::cmp::max(x, w))];
            bct.cmp[e].unwrap()
        };
        let by = if let Some(by) = bct.cmp_node[y] {
            by
        } else {
            let w = bct.g[y][0];
            let e = bct.e2i[&(std::cmp::min(y, w), std::cmp::max(y, w))];
            bct.cmp[e].unwrap()
        };
        let l = lca.lca(bx, by);
        let mut dist = dep[bx] + dep[by] - 2 * dep[l];
        if bct.cmp_node[x].is_some() {
            dist -= 1;
        }
        if bct.cmp_node[y].is_some() {
            dist -= 1;
        }
        puts!("{}\n", dist / 2);
    }
}
