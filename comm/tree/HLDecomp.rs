// https://ei1333.github.io/luzhiled/snippets/tree/heavy-light-decomposition.html
// Verified by: NUPC2017 H
// https://atcoder.jp/contests/njpc2017/submissions/23535017
struct HLDecomp {
    euler: Vec<usize>,
    head: Vec<usize>,
    rev: Vec<usize>,
    par: Vec<usize>,
}

impl HLDecomp {
    fn dfs_sz(v: usize, p: usize, g: &mut [Vec<usize>], sz: &mut [usize],
              par: &mut [usize]) {
        par[v] = p;
        sz[v] = 1;
        if g[v].get(0) == Some(&p) {
            let last = g[v].len() - 1;
            g[v].swap(0, last);
        }
        for i in 0..g[v].len() {
            let to = g[v][i];
            if to == p {
                continue;
            }
            Self::dfs_sz(to, v, g, sz, par);
            sz[v] += sz[to];
            if sz[g[v][0]] < sz[to] {
                g[v].swap(0, i);
            }
        }
    }
    fn dfs_euler(v: usize, par: usize, g: &[Vec<usize>],
                 euler: &mut [usize], count: &mut usize,
                 head: &mut [usize], rev: &mut [usize]) {
        euler[v] = *count;
        *count += 1;
        rev[euler[v]] = v;
        for &to in &g[v] {
            if to == par {
                continue;
            }
            head[to] = if g[v][0] == to { head[v] } else { to };
            Self::dfs_euler(to, v, g, euler, count, head, rev);
        }
    }
    pub fn new(g: &[Vec<usize>]) -> Self {
        let mut g = g.to_vec();
        let n = g.len();
        let mut sz = vec![0; n];
        let mut par = vec![0; n];
        Self::dfs_sz(0, n, &mut g, &mut sz, &mut par);
        let mut euler = vec![0; n];
        let mut count = 0;
        let mut head = vec![0; n];
        let mut rev = vec![0; n];
        Self::dfs_euler(0, n, &g, &mut euler, &mut count, &mut head, &mut rev);
        HLDecomp {
            euler: euler,
            head: head,
            rev: rev,
            par: par,
        }
    }
    #[allow(unused)]
    pub fn get_id(&self, v: usize) -> usize {
        self.euler[v]
    }
    #[allow(unused)]
    pub fn from_id(&self, id: usize) -> usize {
        self.rev[id]
    }
    // M: commutative
    // M must not panic.
    #[allow(unused)]
    pub fn query<T, F: FnMut(usize, usize) -> T, M: Fn(T, T) -> T>(&self, mut u: usize, mut v: usize, mut f: F, mut m: M, e: T, edge: bool) -> T {
        let mut ans = e;
        self.divide(u, v, |l, r| {
            let ptr: *mut T = &mut ans;
            unsafe {
                let val = f(l, r);
                let ans = std::ptr::read(ptr);
                std::ptr::write(ptr, m(ans, val))
            }
        }, edge);
        ans
    }
    pub fn divide<F: FnMut(usize, usize)>(&self, mut u: usize, mut v: usize, mut f: F, edge: bool) {
        let euler = &self.euler;
        let head = &self.head;
        loop {
            if euler[u] > euler[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if head[u] == head[v] {
                break;
            }
            f(euler[head[v]], euler[v] + 1);
            v = self.par[head[v]];
        }
        f(euler[u] + if edge { 1 } else { 0 }, euler[v] + 1);
    }
}
