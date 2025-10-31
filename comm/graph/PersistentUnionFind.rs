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
}
