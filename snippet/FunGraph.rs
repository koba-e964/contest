// if to_cyc[v] == v: dist_cyc[v] = 0
// if to_cyc[v] != v: dist_cyc[v] > 0
// Verified by: https://codeforces.com/contest/1607/submission/134236857
pub struct FunGraph {
    pub cyc: Vec<usize>,
    pub to_cyc: Vec<usize>,
    pub dist_cyc: Vec<usize>,
}

impl FunGraph {
    fn dfs(v: usize, g: &[usize], vis: &mut [bool], ans: &mut FunGraph) {
        let n = g.len();
        let mut st = vec![(v, 0)];
        let mut ret = (0, false, 0);
        while let Some((v, kind)) = st.pop() {
            if kind == 0 {
                if vis[v] {
                    ret = if ans.to_cyc[v] == n {
                        (v, false, 0)
                    } else {
                        (ans.to_cyc[v], true, ans.dist_cyc[v])
                    };
                    continue;
                }
                vis[v] = true;
                st.push((v, 1));
                st.push((g[v], 0));
            } else {
                let (p, confl, len) = ret;
                ret = if !confl {
                    if p == v {
                        let mut path = vec![v];
                        let mut x = g[v];
                        while x != v {
                            path.push(x);
                            x = g[x];
                        }
                        for &a in &path {
                            ans.dist_cyc[a] = 0;
                            ans.cyc[a] = path.len();
                            ans.to_cyc[a] = a;
                        }
                        (p, true, 0)
                    } else {
                        (p, false, 0)
                    }
                } else {
                    ans.to_cyc[v] = p;
                    ans.dist_cyc[v] = len + 1;
                    (p, true, len + 1)
                };
            }
        }
    }
    pub fn new(g: &[usize]) -> Self {
        let n = g.len();
        let mut vis = vec![false; n];
        let cyc = vec![n; n];
        let to_cyc = vec![n; n];
        let dist_cyc = vec![n; n];
        let mut ans = FunGraph {
            cyc: cyc,
            to_cyc: to_cyc,
            dist_cyc: dist_cyc,
        };
        for i in 0..n {
            if !vis[i] {
                Self::dfs(i, &g, &mut vis, &mut ans);
            }
        }
        ans
    }
}
