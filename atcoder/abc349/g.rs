use std::cmp::*;
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

// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
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

// Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
// Verified by https://atcoder.jp/contests/wupc2019/submissions/4540033
fn manacher_inv(r: &[usize], uf: &mut UnionFind) -> Result<(), ()> {
    let n = r.len();
    {
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while j <= r[i] {
                uf.unite(i - j, i + j);
                j += 1;
            }
            let mut k = 1;
            while i >= k && i + k < n && k + r[i - k] + 1 < j {
                if r[i + k] != r[i - k] {
                    return Err(());
                }
                k += 1;
            }
            i += k;
            j -= k;
        }
    }
    Ok(())
}

// Tags: palindrome-radius, inverse-of-manacher, construction
fn main() {
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
        a: [usize; n],
    }
    let mut uf = UnionFind::new(n);
    if manacher_inv(&a, &mut uf).is_err() {
        println!("No");
        return;
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        let r = uf.root(i);
        let x = hm.entry(r).or_insert(r);
        *x = min(*x, i);
    }
    let mut is_min = vec![false; n];
    for (_, &v) in &hm {
        is_min[v] = true;
    }
    let mut edges = vec![vec![]; n];
    for i in 0..n {
        if a[i] < i && a[i] + i + 1 < n {
            let r1 = uf.root(i - a[i] - 1);
            let r2 = uf.root(i + a[i] + 1);
            let x = min(hm[&r1], hm[&r2]);
            let y = max(hm[&r1], hm[&r2]);
            if x == y {
                println!("No");
                return;
            }
            edges[y].push(x);
        }
    }
    let mut val = vec![0; n];
    for i in 0..n {
        if is_min[i] {
            let mut seen = HashSet::new();
            for &to in &edges[i] {
                seen.insert(val[to]);
            }
            let mut mex = 1;
            while seen.contains(&mex) {
                mex += 1;
            }
            val[i] = mex;
        }
    }
    for i in 0..n {
        if !is_min[i] {
            val[i] = val[hm[&uf.root(i)]];
        }
    }
    println!("Yes");
    putvec!(val);
}
