use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// Ported and modified from:
// - <https://blog.tiramister.net/posts/persistent-unionfind/>
// - <https://kopricky.github.io/code/DataStructure_OnGraph/partial_persistent_union_find.html>
// - <https://nyaannyaan.github.io/library/data-structure/rollback-union-find.hpp.html>
// Verified by:
// - <https://yukicoder.me/problems/no/416> <https://yukicoder.me/submissions/1129953>
// - <https://codeforces.com/contest/1444/problem/C> <https://codeforces.com/contest/1444/submission/347096667>
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
    #[allow(dead_code)]
    fn size(&self, x: usize, t: PPUFTime) -> usize {
        let x = self.find(x, t);
        let idx = self.sz[x].binary_search(&(t, i32::max_value())).err().unwrap() - 1;
        self.sz[x][idx].1 as usize
    }
    pub fn now(&self) -> PPUFTime {
        PPUFTime(self.history.len() as i32)
    }
    #[allow(dead_code)]
    pub fn rollback(&mut self, time: PPUFTime) {
        let time = time.0 as usize;
        while self.history.len() > time {
            let (u, v) = self.history.pop().unwrap();
            let _ = self.sz[u].pop();
            self.par[v] = (PPUFTime(i32::max_value()), v as i32);
        }
    }
}

// Tags: partially-persistent-union-find
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        m: usize,
        ab: [(usize1, usize1); m],
        q: usize,
        xyz: [(usize1, usize1, usize); q],
    }
    let mut uf = PartiallyPersistentUnionFind::new(n);
    let mut time = vec![PPUFTime(0); m];
    for (i, (a, b)) in ab.into_iter().enumerate() {
        uf.unite(a, b);
        time[i] = uf.now();
    }
    for (x, y, z) in xyz {
        let mut pass = m;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            let t = time[mid - 1];
            let rx = uf.find(x, t);
            let ry = uf.find(y, t);
            let size = if rx == ry {
                uf.size(rx, t)
            } else {
                uf.size(rx, t) + uf.size(ry, t)
            };
            if size >= z {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        puts!("{pass}\n");
    }
}
