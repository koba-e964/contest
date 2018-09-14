/*
 * Persistent Union Find tree.
 * Reference: https://misteer.hatenablog.com/entry/persistentUF
 */
struct PersistentUnionFind {
    par: Vec<usize>,
    time: Vec<i32>,
    rank: Vec<i32>,
    num: Vec<Vec<(i32, usize)>>, // [(time, size of this component)]
    now: i32,
}
impl PersistentUnionFind {
    fn new(n: usize) -> Self {
        let mut par = vec![0; n];
        let time = vec![i32::max_value(); n];
        let rank = vec![0; n];
        let num = vec![vec![(-1, 1)]; n];
        for i in 0 .. n {
            par[i] = i;
        }
        PersistentUnionFind {
            par: par,
            time: time,
            rank: rank,
            num: num,
            now: 0
        }
    }
    fn root(&self, mut x: usize, t: i32) -> usize {
        loop {
            if self.time[x] > t { return x; }
            x = self.par[x];
        }
    }
    // returns the current time
    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) -> i32 {
        let mut now = self.now;
        let mut x = self.root(x, now);
        let mut y = self.root(y, now);
        if x == y { return now; }
        now += 1;
        if self.rank[x] <= self.rank[y] { std::mem::swap(&mut x, &mut y); }
        self.par[y] = x;
        self.time[y] = now;
        self.rank[x] = std::cmp::max(self.rank[x], self.rank[y] + 1);
        let size0 = self.num[x].last().unwrap().1;
        let size1 = self.num[y].last().unwrap().1;
        self.num[x].push((now, size0 + size1));
        self.now = now;
        now
    }
    #[allow(dead_code)]
    fn size(&self, x: usize, t: i32) -> usize {
        let x = self.root(x, t);
        let idx = self.num[x].binary_search(&(t, usize::max_value())).err().unwrap() - 1;
        self.num[x][idx].1
    }
}
