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

// Strong connected components.
// Verified by: yukicoder No.470 (http://yukicoder.me/submissions/145785)
//              ABC214-H (https://atcoder.jp/contests/abc214/submissions/25082618)
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
        let mut ret = vec![vec![]; ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[i]].push(self.cmp[to]);
                }
            }
        }
        ret.into_iter().map(|mut v| {
            v.sort_unstable(); v.dedup();
            v
        }).collect()
    }
    #[allow(dead_code)]
    fn rdag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![vec![]; ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[to]].push(self.cmp[i]);
                }
            }
        }
        ret.into_iter().map(|mut v| {
            v.sort_unstable(); v.dedup();
            v
        }).collect()
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

fn primes(lim: usize) -> Vec<bool> {
    if lim <= 1 {
        return vec![];
    }
    let mut pr = vec![true; lim + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..=lim {
        if !pr[i] {
            continue;
        }
        for j in 2..=lim / i {
            pr[i * j] = false;

        }
    }
    pr
}

fn concat(mut a: usize, b: usize) -> usize {
    let mut v = b;
    while v > 0 {
        v /= 10;
        a *= 10;
    }
    a + b
}

// x = 0 => not a, x = 1 => a
fn make_lit(a: usize, x: usize) -> i32 {
    let a = a as i32 + 1;
    if x == 0 {
        -a
    } else {
        a
    }
}

// Tags: 2-sat
fn main() {
    input! {
        n: usize,
        ab: [[usize; 2]; n],
    }
    let pr = primes(1_000_000);
    // in S -> true, in T -> false
    let mut cons = vec![];
    for i in 0..n {
        for j in 0..n {
            for x in 0..2 {
                for y in 0..2 {
                    let v = concat(ab[i][x], ab[j][y]);
                    if pr[v] {
                        cons.push((make_lit(i, x), make_lit(j, 1 - y)));
                    }
                }
            }
        }
    }
    println!("{}", if two_sat(n, &cons).is_some() { "Yes" } else { "No" });
}
