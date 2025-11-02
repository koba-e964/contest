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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Ported and modified from:
// - <https://blog.tiramister.net/posts/persistent-unionfind/>
// - <https://kopricky.github.io/code/DataStructure_OnGraph/partial_persistent_union_find.html>
// - <https://nyaannyaan.github.io/library/data-structure/rollback-union-find.hpp.html>
// Verified by:
// - <https://yukicoder.me/problems/no/416> <https://yukicoder.me/submissions/1129953>
struct PartiallyPersistentUnionFind {
    history: Vec<(usize, usize)>,
    par: Vec<(PPUFTime, i32)>,
    sz: Vec<Vec<(PPUFTime, i32)>>,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct PPUFTime(i32);

impl PartiallyPersistentUnionFind {
    pub fn new(n: usize) -> Self {
        let mut par = Vec::with_capacity(n);
        let mut sz = vec![vec![]; n];
        for i in 0..n {
            par.push((PPUFTime(i32::max_value()), i as i32));
            sz[i].push((PPUFTime(-1), 1));
        }
        Self {
            history: vec![],
            par,
            sz,
        }
    }
    pub fn find(&self, mut u: usize, time: PPUFTime) -> usize {
        while self.par[u].0 <= time { u = self.par[u].1 as usize; }
        u
    }
    #[allow(unused)]
    pub fn same(&self, u: usize, v: usize, time: PPUFTime) -> bool {
        self.find(u, time) == self.find(v, time)
    }
    pub fn unite(&mut self, mut u: usize, mut v: usize) -> bool {
        let time = PPUFTime(self.history.len() as i32 + 1);
        u = self.find(u, time);
        v = self.find(v, time);
        if u == v { return false; }
        if self.sz[u].last().unwrap().1 < self.sz[v].last().unwrap().1 {
            std::mem::swap(&mut u, &mut v);
        }
        self.par[v] = (time, u as i32);
        let whole = self.sz[u].last().unwrap().1 + self.sz[v].last().unwrap().1;
        self.sz[u].push((time, whole));
        self.history.push((u, v));
        true
    }
    pub fn now(&self) -> PPUFTime {
        PPUFTime(self.history.len() as i32)
    }
    pub fn rollback(&mut self, time: PPUFTime) {
        let time = time.0 as usize;
        while self.history.len() > time {
            let (u, v) = self.history.pop().unwrap();
            let _ = self.sz[u].pop();
            self.par[v] = (PPUFTime(i32::max_value()), v as i32);
        }
    }
}

// Tags: partially-persistent-union-find, union-find-with-rollback
fn main() {
    input! {
        n: usize, m: usize, k: usize,
        c: [usize1; n],
        ab: [(usize1, usize1); m],
    }
    let mut uf = PartiallyPersistentUnionFind::new(2 * n);
    let mut diff = HashMap::new();
    for &(a, b) in &ab {
        if c[a] == c[b] {
            uf.unite(a, n + b);
            uf.unite(b, n + a);
        } else {
            let mut a = a;
            let mut b = b;
            if c[a] > c[b] {
                std::mem::swap(&mut a, &mut b);
            }
            diff.entry((c[a], c[b])).or_insert(vec![]).push((a, b));
        }
    }
    let start = uf.now();
    let mut rep = vec![vec![]; k];
    for i in 0..n {
        rep[c[i]].push(i);
    }
    for i in 0..k {
        rep[i].sort_unstable(); rep[i].dedup();
    }
    let mut valid_k = 0i64;
    let mut is_valid = vec![false; k];
    for i in 0..k {
        is_valid[i] = true;
        for &r in &rep[i] {
            if uf.same(r, n + r, start) {
                is_valid[i] = false;
                break;
            }
        }
        if is_valid[i] {
            valid_k += 1;
        }
    }
    let mut ans = valid_k * (valid_k - 1) / 2;
    for ((ca, cb), es) in diff {
        if !is_valid[ca] || !is_valid[cb] {
            continue;
        }
        let mut some = vec![];
        let mut whack = false;
        for (u, v) in es {
            uf.unite(u, v + n);
            uf.unite(v, u + n);
            some.push(u);
        }
        for s in some {
            if uf.same(s, s + n, uf.now()) {
                whack = true;
                break;
            }
        }
        if whack {
            ans -= 1;
        }
        uf.rollback(start);
    }
    println!("{ans}");
}
