// Plain HL decomposition.
// Verified by: ABC269-Ex (https://atcoder.jp/contests/abc269/submissions/39116328)
pub struct PlainHLD {
    pub st: Vec<usize>,
    pub jmp: Vec<usize>,
    pub dep: Vec<usize>,
}

impl PlainHLD {
    // For each node, make the heavy child the first child.
    fn dfs_left(ch: &mut [Vec<usize>], v: usize, sz: &mut [usize],
                dep: &mut [usize]) {
        let mut stack = vec![(v, 0, 0)];
        while let Some((v, d, kind)) = stack.pop() {
            if kind == 0 {
                dep[v] = d;
                stack.push((v, d, 1));
                for i in 0..ch[v].len() {
                    let w = ch[v][i];
                    stack.push((w, d + 1, 0));
                }
            } else {
                let mut s = 1;
                for i in 0..ch[v].len() {
                    let w = ch[v][i];
                    s += sz[w];
                    if sz[w] > sz[ch[v][0]] {
                        ch[v].swap(i, 0);
                    }
                }
                sz[v] = s;
            }
        }
    }
    fn dfs(ch: &[Vec<usize>], st: &mut [usize], v: usize, jmp: &mut [usize]) {
        let mut stack = vec![v];
        let mut cnt = 0;
        while let Some(v) = stack.pop() {
            st[v] = cnt;
            cnt += 1;
            if ch[v].len() >= 1 {
                jmp[ch[v][0]] = jmp[v];
            }
            for &w in &ch[v] {
                stack.push(w);
            }
        }
    }
    pub fn new(ch: &mut [Vec<usize>], root: usize) -> Self {
        let n = ch.len();
        let mut st = vec![0; n];
        let mut sz = vec![0; n];
        let mut jmp = vec![0; n];
        let mut dep = vec![0; n];
        Self::dfs_left(ch, root, &mut sz, &mut dep);
        for i in 0..n {
            jmp[i] = i;
        }
        Self::dfs(ch, &mut st, root, &mut jmp);
        PlainHLD {
            st: st,
            jmp: jmp,
            dep: dep,
        }
    }
}
