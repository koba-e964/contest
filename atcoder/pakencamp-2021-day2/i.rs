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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, par: usize, p: &mut [usize], g: &[Vec<usize>], ord: &mut Vec<usize>) {
    p[v] = par;
    for &w in &g[v] {
        if w == par { continue; }
        dfs(w, v, p, g, ord);
    }
    ord.push(v);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [i32; n - 1],
        b: [i32; n - 1],
    }
    let mut uf = UnionFind::new(n - 1);
    let mut g = vec![vec![]; n - 1];
    for i in 2..n + 1 {
        for j in 2..n / i + 1 {
            if uf.is_same_set(i - 2, i * j - 2) { continue; }
            uf.unite(i - 2, i * j - 2);
            g[i - 2].push(i * j - 2);
            g[i * j - 2].push(i - 2);
        }
    }
    let mut par = vec![n + 1; n - 1];
    let mut ord = vec![vec![]; n - 1];
    for i in 0..n - 1 {
        if par[i] == n + 1 {
            dfs(i, n, &mut par, &g, &mut ord[i]);
        }
    }
    let mut fst = vec![vec![]; n - 1];
    let mut snd = vec![vec![]; n - 1];
    let mut fst_i = vec![vec![]; n - 1];
    let mut snd_i = vec![vec![]; n - 1];
    for i in 0..n - 1 {
        let r = uf.root(i);
        fst[r].push(a[i]);
        snd[r].push(b[i]);
        fst_i[r].push((a[i], i));
        snd_i[r].push((b[i], i));
    }
    let mut ops = vec![];
    let mut p = vec![0; n - 1];
    let mut invp = vec![0; n - 1];
    for i in 0..n - 1 {
        if par[i] != n { continue; }
        let r = uf.root(i);
        fst.swap(i, r);
        snd.swap(i, r);
        fst_i.swap(i, r);
        snd_i.swap(i, r);
        fst[i].sort_unstable();
        snd[i].sort_unstable();
        if fst[i] != snd[i] {
            puts!("-1\n");
            return;
        }
        fst_i[i].sort_unstable();
        snd_i[i].sort_unstable();
        for j in 0..fst_i[i].len() {
            let x = fst_i[i][j].1;
            let y = snd_i[i][j].1;
            p[x] = y;
            invp[y] = x;
        }
        for &v in &ord[i] {
            let mut cur = invp[v];
            while cur != i {
                // swap cur and par[cur]
                ops.push((cur, par[cur]));
                invp.swap(p[cur], p[par[cur]]);
                p.swap(cur, par[cur]);
                cur = par[cur];
            }
            cur = v;
            let mut trace = vec![];
            while cur != i {
                trace.push(cur);
                cur = par[cur];
            }
            trace.push(i);
            for j in (0..trace.len() - 1).rev() {
                // swap cur and par[cur]
                ops.push((trace[j], trace[j + 1]));
                invp.swap(p[trace[j]], p[trace[j + 1]]);
                p.swap(trace[j], trace[j + 1]);
            }
        }
    }
    // eprintln!("{:?}", p);
    assert!(ops.len() <= 1_000_000);
    puts!("{}\n", ops.len());
    for (x, y) in ops {
        let (x, y) = if x < y { (x, y) } else { (y, x) };
        puts!("{} {}\n", x + 2, y + 2);
    }
}
