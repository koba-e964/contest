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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

/// Treap (balanced binary search tree)
/// Reference: https://www.slideshare.net/iwiwi/2-12188757
/// Verified by: ARC061-D (http://arc061.contest.atcoder.jp/submissions/1172709)
/// 2150ms for n = 9*10^5, maybe a bit slow for an O(n * log(n))-time algorithm...
#[derive(Clone, Debug)]
enum Treap<T> {
    Bin(
        usize, // size
        i64, // priority
        T, // value
        Box<Treap<T>>, // left
        Box<Treap<T>>, // right
    ),
    Tip,
}

impl<T: Ord> Treap<T> {
    pub fn new() -> Self { Treap::Tip }
    pub fn singleton(v: T, pri: i64) -> Self {
        use Treap::*;
        Bin(1, pri, v, Box::new(Tip), Box::new(Tip))
    }
    pub fn size(&self) -> usize {
        use Treap::*;
        match *self {
            Tip => 0,
            Bin(t, _,  _, _, _) => t,
        }
    }
    // Merges two BST. Their ownership is taken.
    pub fn merge(left: Self, right: Self) -> Self {
        use Treap::*;
        match (left, right) {
            (Tip, Tip) => Tip,
            (Tip, x) => x,
            (x, Tip) => x,
            (Bin(lsize, lpri, lelem, lleft, lright),
             Bin(rsize, rpri, relem, rleft, rright)) => {
                if lpri > rpri {
                    let right = Bin(rsize, rpri, relem, rleft, rright);
                    let mut ret = Bin(lsize, lpri, lelem, lleft,
                                  Box::new(Self::merge(*lright, right)));
                    ret.update();
                    ret
                } else {
                    let left = Bin(lsize, lpri, lelem, lleft, lright);
                    let mut ret = Bin(rsize, rpri, relem,
                                      Box::new(Self::merge(left, *rleft)),
                                      rright);
                    ret.update();
                    ret
                }
            }
        }
    }
    pub fn split(self, k: usize) -> (Self, Self) {
        use Treap::*;
        match self {
            Tip => (Tip, Tip),
            Bin(size, pri, elem, left, right) => {
                if k <= left.size() {
                    let (sl, sr) = Self::split(*left, k);
                    let mut ret = Bin(size, pri, elem, Box::new(sr), right);
                    ret.update();
                    (sl, ret)
                } else {
                    let (sl, sr) = Self::split(*right, k - left.size() - 1);
                    let mut ret = Bin(size, pri, elem, left, Box::new(sl));
                    ret.update();
                    (ret, sr)
                }
            }
        }
    }
    fn update(&mut self) {
        use Treap::*;
        match *self {
            Tip => (),
            Bin(ref mut lsize, ref _pri, ref _elem, ref left, ref right) => {
                *lsize = left.size() + right.size() + 1;
            },
        }
    }
    fn insert_at(self, v: T, pri: i64, k: usize) -> Self {
        use Treap::*;
        // Speed up: compare the priority
        match self {
            Tip => Self::singleton(v, pri),
            Bin(size, spri, elem, left, right) => {
                let lsize = left.size();
                if spri <= pri {
                    let cself = Bin(size, spri, elem, left, right);
                    let (left, right) = cself.split(k);
                    return Bin(size + 1, pri, v,
                               Box::new(left), Box::new(right));
                }
                if k < lsize {
                    return Bin(size + 1, spri, elem,
                               Box::new((*left).insert_at(v, pri, k)),
                               right);
                }
                if k >= lsize + 1 {
                    return Bin(size + 1, spri, elem,
                               left,
                               Box::new((*right)
                                        .insert_at(v, pri, k - lsize - 1)));
                }
                let cself = Bin(size, spri, elem, left, right);
                let sing = Self::singleton(v, pri);
                let (left, right) = cself.split(k);
                let tmp = Self::merge(left, sing);
                Self::merge(tmp, right)
            }
        }
    }
    fn erase_at(self, k: usize) -> Self {
        use Treap::*;
        match self {
            Tip => Tip,
            Bin(size, pri, elem, left, right) => {
                if k < left.size() {
                    return Bin(size - 1, pri, elem,
                               Box::new((*left).erase_at(k)), right);
                }
                if k == left.size() {
                    return Self::merge(*left, *right); // hit
                }
                let lsize = left.size();
                return Bin(size - 1, pri, elem,
                           left,
                           Box::new((*right).erase_at(k - lsize - 1)));
            }
        }
    }
    fn find_index(&self, t: &T) -> (usize, bool) {
        use Treap::*;
        use std::cmp::Ordering;
        let mut offset = 0;
        let mut tap = self;
        loop {
            match *tap {
                Tip => return (offset, false),
                Bin(_, _, ref elem, ref left, ref right) => {
                    match elem.cmp(t) {
                        Ordering::Equal => return (offset + left.size(), true),
                        Ordering::Greater =>
                            tap = left,
                        Ordering::Less => {
                            offset += left.size() + 1;
                            tap = right;
                        },
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    fn insert(self, v: T, pri: i64) -> Self {
        let (idx, found) = self.find_index(&v);
        if found {
            self
        } else {
            self.insert_at(v, pri, idx)
        }
    }
    #[allow(dead_code)]
    fn erase(self, v: &T) -> Self {
        let (idx, found) = self.find_index(v);
        if found {
            self.erase_at(idx)
        } else {
            self
        }
    }
    #[allow(dead_code)]
    fn at(&self, k: usize) -> Option<&T> {
        use Treap::*;
        match self {
            &Tip => None,
            &Bin(_size, _pri, ref elem, ref left, ref right) => {
                if k < left.size() {
                    return left.at(k);
                }
                if k == left.size() {
                    return Some(&elem); // hit
                }
                let lsize = left.size();
                return right.at(k - lsize - 1);
            }
        }
    }
    fn into_vec_sub(self, vec: &mut Vec<T>) {
        use Treap::*;
        match self {
            Tip => (),
            Bin(_, _, elem, left, right) => {
                left.into_vec_sub(vec);
                vec.push(elem);
                right.into_vec_sub(vec);
            }
        }
    }
    #[allow(dead_code)]
    pub fn into_vec(self) -> Vec<T> {
        let mut ret = Vec::new();
        self.into_vec_sub(&mut ret);
        ret
    }
    #[allow(dead_code)]
    fn erase_at_mut(&mut self, k: usize) {
        let tmp = std::mem::replace(self, Treap::Tip);
        let tmp = tmp.erase_at(k);
        std::mem::replace(self, tmp);
    }
    #[allow(dead_code)]
    fn insert_mut(&mut self, v: T, pri: i64) {
        let tmp = std::mem::replace(self, Treap::Tip);
        let tmp = tmp.insert(v, pri);
        std::mem::replace(self, tmp);
    }
    #[allow(dead_code)]
    fn erase_mut(&mut self, v: &T) {
        let tmp = std::mem::replace(self, Treap::Tip);
        let tmp = tmp.erase(v);
        std::mem::replace(self, tmp);
    }
}

fn centroid_decompose(g: &[Vec<(usize, i64)>]) -> Vec<(usize, usize, i64)> {
    fn find_subtree_sizes(g: &[Vec<(usize, i64)>], v: usize, par: usize,
                          dp: &mut [usize], vis: &[bool]) {
        let mut sum = 1;
        for &(w, _) in &g[v] {
            if par == w || vis[w] { continue; }
            find_subtree_sizes(g, w, v, dp, vis);
            sum += dp[w];
        }
        dp[v] = sum;
    }
    fn centroid_decompose_inner(g: &[Vec<(usize, i64)>], v: usize, par: usize,
                                cost: i64, edges: &mut Vec<(usize, usize, i64)>,
                                dp: &mut [usize], vis: &mut [bool]) {
        let n = g.len();
        find_subtree_sizes(g, v, n, dp, vis);
        let (cent, dist) = {
            let sz = dp[v];
            let find_centroid = |mut v: usize, mut par: usize| {
                let mut dist = 0;
                loop {
                    let mut has_majority = false;
                    for &(w, c) in &g[v] {
                        if par == w || vis[w] { continue; }
                        if dp[w] > sz / 2 {
                            dist += c;
                            par = v;
                            v = w;
                            has_majority = true;
                            break;
                        }
                    }
                    if !has_majority {
                        return (v, dist);
                    }
                }
            };
            find_centroid(v, n)
        };
        let g_cent = g[cent].clone();
        if par < n {
            edges.push((par, cent, dist + cost));
        }
        // v was selected as a centroid
        // and will be ignored in the following decomposition procedure
        vis[cent] = true;
        for &(w, c) in &g_cent {
            if !vis[w] {
                centroid_decompose_inner(g, w, cent, c, edges, dp, vis);
            }
        }
    }
    let n = g.len();
    let mut edges = vec![];
    // This Vec is reused many times
    let mut dp = vec![0; n];
    let mut vis = vec![false; n];
    centroid_decompose_inner(&g, 0, n, 0, &mut edges, &mut dp, &mut vis);
    edges
}

const B: usize = 17;

fn init_lca_dfs(g: &[Vec<(usize, i64)>], v: usize, par: &mut [usize],
                dep: &mut [usize], dep_dist: &mut [i64]) {
    for &(w, c) in &g[v] {
        if w == par[v] { continue; }
        par[w] = v;
        dep[w] = dep[v] + 1;
        dep_dist[w] = dep_dist[v] + c;
        init_lca_dfs(g, w, par, dep, dep_dist);
    }
}

fn init_lca(g: &[Vec<(usize, i64)>]) -> (Vec<usize>, Vec<i64>, Vec<Vec<usize>>) {
    let n = g.len();
    let mut lca = vec![vec![n; n]; B];
    let mut dep = vec![0; n];
    let mut dep_dist = vec![0; n];
    let mut par = vec![n; n];
    init_lca_dfs(g, 0, &mut par, &mut dep, &mut dep_dist);
    for v in 0..n {
        lca[0][v] = par[v];
    }
    for i in 0..B - 1 {
        for v in 0..n {
            let w = lca[i][v];
            lca[i + 1][v] = if w >= n {
                n
            } else {
                lca[i][w]
            };
        }
    }
    (dep, dep_dist, lca)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let mut x = 0x15262627i64;
    let a = 0x245711;
    let b = 0x13331;
    let mut next = || {
        x = x.wrapping_mul(a).wrapping_add(b);
        x
    };
    input! {
        n: usize,
        m: usize,
        abd: [(usize1, usize1, i64); n - 1],
        secx: [(i64, i64, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, d) in &abd {
        g[a].push((b, d));
        g[b].push((a, d));
    }
    // Construct centroid tree
    let edges = centroid_decompose(&g);
    let mut tree = vec![vec![]; n];
    let mut par = vec![n; n];
    for &(p, child, cost) in &edges {
        tree[p].push((child, cost));
        par[child] = p;
    }
    let (dep, dep_dist, lca_aux) = init_lca(&g);
    let lca = |mut x: usize, mut y: usize| {
        if dep[x] > dep[y] {
            std::mem::swap(&mut x, &mut y);
        }
        for i in (0..B).rev() {
            if dep[y] >= dep[x] + (1 << i) {
                y = lca_aux[i][y];
            }
        }
        assert_eq!(dep[x], dep[y]);
        if x == y {
            return x;
        }
        for i in (0..B).rev() {
            if dep[x] <= 1 << i { continue; }
            if lca_aux[i][x] == lca_aux[i][y] { continue; }
            x = lca_aux[i][x];
            y = lca_aux[i][y];
            assert_ne!(x, y);
        }
        x = lca_aux[0][x];
        y = lca_aux[0][y];
        assert_eq!(x, y);
        x
    };
    let gdist = |x: usize, y: usize| {
        let l = lca(x, y);
        let v = dep_dist[x] + dep_dist[y] - 2 * dep_dist[l];
        v
    };
    let mut secx = secx;
    secx.sort();
    let mut dp = vec![0; m];
    let mut pool = vec![Treap::new(); n];
    // set should be, and will be, strictly increasing.
    fn update_element<F>(set: &mut Treap<(i64, i64)>, a: i64, b: i64, next: &mut F)
    where F: FnMut() -> i64 {
        let (idx, _) = set.find_index(&(a, b));
        if idx >= 1 {
            let &(_, pro) = set.at(idx - 1).unwrap();
            if pro >= b { return; }
        }
        set.insert_mut((a, b), next());
        while let Some(&(_cost, pro)) = set.at(idx + 1) {
            if b >= pro {
                set.erase_at_mut(idx + 1);
            } else {
                break;
            }
        }
    }
    for i in 0..m {
        let (s, e, c, x) = secx[i];
        let mut cur = c;
        loop {
            let dist = gdist(cur, c);
            let (idx, _) = pool[cur].find_index(&(s - dist + 1, 0));
            if idx >= 1 {
                let &(last, profit) = pool[cur].at(idx - 1).unwrap();
                assert!(last + dist <= s);
                dp[i] = max(dp[i], profit);
            }
            if par[cur] >= n { break; }
            cur = par[cur];
        }
        dp[i] += x;
        let mut cur = c;
        loop {
            let dist = gdist(cur, c);
            update_element(&mut pool[cur], dist + e, dp[i], &mut next); 
            if par[cur] >= n { break; }
            cur = par[cur];
        }
    }
    let ma: i64 = dp.into_iter().max().unwrap();
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
