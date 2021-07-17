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

/**
 * Strong connected components.
 * Verified by: yukicoder No.470 (http://yukicoder.me/submissions/145785)
 */
struct SCC {
    n: usize,
    ncc: usize,
    g: Vec<Vec<usize>>, // graph in adjacent list
    rg: Vec<Vec<usize>>, // reverse graph
    cmp: Vec<usize>, // topological order
}

impl SCC {
    fn new(n: usize) -> Self {
        SCC {
            n: n,
            ncc: n + 1,
            g: vec![Vec::new(); n],
            rg: vec![Vec::new(); n],
            cmp: vec![0; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize) {
        self.g[from].push(to);
        self.rg[to].push(from);
    }
    fn dfs(&self, v: usize, used: &mut [bool], vs: &mut Vec<usize>) {
        used[v] = true;
        for &w in self.g[v].iter() {
            if !used[w] {
               self.dfs(w, used, vs);
            }
        }
        vs.push(v);
    }
    fn rdfs(&self, v: usize, k: usize,
            used: &mut [bool], cmp: &mut [usize]) {
        used[v] = true;
        cmp[v] = k;
        for &w in self.rg[v].iter() {
            if !used[w] {
                self.rdfs(w, k, used, cmp);
            }
        }
    }
    fn scc(&mut self) -> usize {
        let n = self.n;
        let mut used = vec![false; n];
        let mut vs = Vec::new();
        let mut cmp = vec![0; n];
        for v in 0 .. n {
            if !used[v] { self.dfs(v, &mut used, &mut vs); }
        }
        for u in used.iter_mut() {
            *u = false;
        }
        let mut k = 0;
        for &t in vs.iter().rev() {
            if !used[t] { self.rdfs(t, k, &mut used, &mut cmp); k += 1; }
        }
        self.ncc = k;
        self.cmp = cmp;
        k
    }
    #[allow(dead_code)]
    fn top_order(&self) -> Vec<usize> {
        assert!(self.ncc <= self.n);
        self.cmp.clone()
    }
    /*
     * Returns a dag whose vertices are scc's, and whose edges are those of the original graph.
     */
    #[allow(dead_code)]
    fn dag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![HashSet::new(); ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[i]].insert(self.cmp[to]);
                }
            }
        }
        ret.into_iter().map(|set| set.into_iter().collect()).collect()
    }
    #[allow(dead_code)]
    fn rdag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![HashSet::new(); ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[to]].insert(self.cmp[i]);
                }
            }
        }
        ret.into_iter().map(|set| set.into_iter().collect()).collect()
    }
}

/**
 * 2-SAT solver.
 * n: the number of variables (v_1, ..., v_n)
 * cons: constraints, given in 2-cnf
 * i (1 <= i <= n) means v_i, -i (1 <= i <= n) means not v_i.
 * Returns: None if there's no assignment that satisfies cons.
 * Otherwise, it returns an assignment that safisfies cons. (true: true, false: false)
 * Dependencies: SCC.rs
 * Verified by: Codeforces #400 D
 *              (http://codeforces.com/contest/776/submission/24957215)
 */
fn two_sat(n: usize, cons: &[(i32, i32)]) -> Option<Vec<bool>> {
    let mut scc = SCC::new(2 * n);
    let ni = n as i32;
    for &(c1, c2) in cons.iter() {
        let x = if c1 > 0 {
            c1 - 1 + ni
        } else {
            -c1 - 1
        } as usize;
        let y = if c2 > 0 {
            c2 - 1
        } else {
            -c2 - 1 + ni
        } as usize;
        scc.add_edge(x, y);
        scc.add_edge((y + n) % (2 * n), (x + n) % (2 * n));
    }
    scc.scc();
    let mut result = vec![false; n];
    let top_ord = scc.top_order();
    for i in 0 .. n {
        if top_ord[i] == top_ord[i + n] {
            return None;
        }
        result[i] = top_ord[i] > top_ord[i + n];
    }
    Some(result)
}

// Tags: scc, 2sat, segment-tree
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    const W: usize = 2_000_001;
    let mut pr = vec![true; W];
    let mut fac = vec![0; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] {
            continue;
        }
        fac[i] = i;
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
            fac[i * j] = i;
        }
    }
    let facs = |mut v: usize| {
        let mut ans = vec![];
        while v > 1 {
            let p = fac[v];
            ans.push(p);
            while v % p == 0 {
                v /= p;
            }
        }
        ans
    };
    let mut ps = vec![];
    for i in 2..W {
        if pr[i] {
            ps.push(i);
        }
    }
    let m = ps.len();
    eprintln!("|ps| = {}", m);
    let mut pi = vec![0; W];
    for i in 2..W {
        pi[i] = pi[i - 1] + if pr[i] { 1 } else { 0 };
    }
    fn not((a, b): (usize, i32)) -> (usize, i32) {
        (a, 1 - b)
    }
    let mut vs = vec![vec![]; m];
    for i in 0..n {
        let (a, b) = ab[i];
        let af = facs(a);
        let bf = facs(b);
        for p in af {
            vs[pi[p] - 1].push((i, 0));
        }
        for p in bf {
            vs[pi[p] - 1].push((i, 1));
        }
    }
    let mut gen = n;
    let mut cons = vec![];
    for i in 0..m {
        let mut cur = vs[i].clone();
        while cur.len() > 1 {
            let mut half = vec![];
            for i in 0..cur.len() / 2 {
                let v = gen;
                gen += 1;
                cons.push((not(cur[2 * i]), not(cur[2 * i + 1])));
                half.push((v, 0));
                for j in 0..2 {
                    // cur[2 * i + j] => v
                    cons.push((not(cur[2 * i + j]), (v, 0)));
                }
            }
            if cur.len() % 2 == 1 {
                half.push(cur.pop().unwrap());
            }
            cur = half;
        }
    }
    let mut conv = vec![];
    fn cc((a, k): (usize, i32)) -> i32 {
        if k == 0 {
            a as i32 + 1
        } else {
            -(a as i32 + 1)
        }
    }
    for (v1, v2) in cons {
        conv.push((cc(v1), cc(v2)));
    }
    let sol = two_sat(gen, &conv);
    println!("{}", if sol.is_some() { "Yes" } else { "No" });
}
