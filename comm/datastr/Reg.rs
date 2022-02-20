// Registry.
// Verified by: yukicoder No.1847 (https://yukicoder.me/submissions/741000)
#[derive(Clone, Debug)]
struct Reg<T> {
    a: Vec<T>,
    inv: std::collections::HashMap<T, usize>,
}

impl<T: std::hash::Hash + Eq + Clone> Reg<T> {
    fn get(&mut self, t: &T) -> usize {
        if !self.inv.contains_key(t) {
            let idx = self.a.len();
            self.a.push(t.clone());
            self.inv.insert(t.clone(), idx);
        }
        self.inv[t]
    }
    // init must have distinct elements.
    fn init<F>(&mut self, init: &[T], f: F) -> Vec<Vec<i64>>
    where F: Fn(T) -> Vec<T> {
        let mut que = std::collections::VecDeque::new();
        for t in init {
            let idx = self.get(t);
            que.push_back(idx);
        }
        let mut n = self.a.len();
        let mut vis = vec![false; n];
        let mut to = vec![vec![]; n];
        while let Some(v) = que.pop_front() {
            if vis[v] { continue; }
            let ans = f(self.a[v].clone());
            let mut entries = vec![];
            for elem in ans {
                let idx = self.get(&elem);
                entries.push(idx);
                if n <= idx {
                    // A newly created entry.
                    n = self.a.len();
                    vis.resize(n, false);
                    to.resize(n, vec![]);
                    que.push_back(idx);
                }
            }
            vis[v] = true;
            to[v] = entries;
        }
        let mut ans = vec![vec![0; n]; n];
        for i in 0..n {
            for &e in &to[i] {
                ans[i][e] += 1;
            }
        }
        ans
    }
    pub fn find_index(&self, t: &T) -> usize {
        self.inv[t]
    }
    // init must have distinct elements.
    pub fn new<F>(init: &[T], f: F) -> (Self, Vec<Vec<i64>>)
    where F: Fn(T) -> Vec<T> {
        let mut me = Reg { a: vec![], inv: std::collections::HashMap::default() };
        let res = me.init(init, f);
        (me, res)
    }
}
