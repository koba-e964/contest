pub struct LCA {
    st: Vec<usize>,
    par: Vec<usize>,
    jmp: Vec<usize>,
    dep: Vec<usize>,
}

// Constant-factor speedup used in https://codeforces.com/contest/1083/submission/46874242.
// Base on HL-decomposition.
// par[root] = root should hold.
// These functions use O(n) stack space.
// Verified by https://codeforces.com/contest/1083/submission/51934575.
impl LCA {
    // For each node, make the most heavy child the first child.
    fn dfs_left(ch: &mut [Vec<usize>], v: usize, sz: &mut [usize],
                dep: &mut [usize], d: usize) {
        dep[v] = d;
        let mut s = 1;
        for i in 0..ch[v].len() {
            let w = ch[v][i];
            Self::dfs_left(ch, w, sz, dep, d + 1);
            s += sz[w];
            if sz[w] > sz[ch[v][0]] {
                ch[v].swap(i, 0);
            }
        }
        sz[v] = s;
    }
    fn dfs(ch: &[Vec<usize>], st: &mut [usize], v: usize,
           cnt: &mut usize, jmp: &mut [usize]) {
        st[v] = *cnt;
        *cnt += 1;
        if ch[v].len() >= 1 {
            jmp[ch[v][0]] = jmp[v];
        }
        for &w in &ch[v] {
            Self::dfs(ch, st, w, cnt, jmp);
        }
    }
    pub fn new(ch: &mut [Vec<usize>], par: &[usize], root: usize) -> Self {
        let n = ch.len();
        let mut st = vec![0; n];
        let mut cnt = 0;
        let mut sz = vec![0; n];
        let mut jmp = vec![0; n];
        let mut dep = vec![0; n];
        Self::dfs_left(ch, root, &mut sz, &mut dep, 0);
        for i in 0..n {
            jmp[i] = i;
        }
        Self::dfs(ch, &mut st, root, &mut cnt, &mut jmp);
        LCA {
            st: st,
            par: par.to_vec(),
            jmp: jmp,
            dep: dep,
        }
    }
    pub fn lca(&self, mut x: usize, mut y: usize) -> usize {
        let jmp = &self.jmp;
        let st = &self.st;
        while jmp[x] != jmp[y] {
            if st[x] < st[y] {
                std::mem::swap(&mut x, &mut y);
            }
            x = self.par[jmp[x]];
        }
        if st[x] < st[y] {
            x
        } else {
            y
        }
    }
}
