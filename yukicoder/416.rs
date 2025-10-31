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
}

// Tags: partially-persistent-union-find
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize, q: usize,
        ab: [(usize1, usize1); m],
        cd: [(usize1, usize1); q],
    }
    let mut broken = HashSet::new();
    for i in 0..q {
        broken.insert(cd[i]);
    }
    let mut uf = PartiallyPersistentUnionFind::new(n);
    for &e in &ab {
        if !broken.contains(&e) {
            uf.unite(e.0, e.1);
        }
    }
    let mut times = vec![uf.now()];
    for &(c, d) in cd.iter().rev() {
        uf.unite(c, d);
        let time = uf.now();
        times.push(time);
    }
    assert_eq!(times.len(), q + 1);
    for i in 1..n {
        let mut pass = q as i32 + 1;
        let mut fail = -1;
        while pass - fail > 1 {
            let mid = (pass - fail) / 2 + fail;
            if uf.same(0, i, times[mid as usize]) {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        // connected right after cd[q - 1 - (pass - 1)] is added
        // q - 1 - (pass - 1) + 1 is the answer unless pass == 0 (connected from the beginning)
        let ans = if pass == 0 {
            -1
        } else {
            q as i32 + 1 - pass
        };
        puts!("{ans}\n");
    }
}
