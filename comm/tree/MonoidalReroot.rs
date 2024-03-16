// Ported from https://trap.jp/post/1702/
// Verified by: https://yukicoder.me/submissions/961222
trait Monoid {
    type V;
    type E: Clone;
    fn add(a: &Self::E, b: &Self::E) -> Self::E;
    fn e() -> Self::E;
    fn put_edge(a: &Self::V, eidx: EIdx) -> Self::E;
    fn put_vertex(a: &Self::E, vidx: usize) -> Self::V;
}

struct MonoidalReroot<M: Monoid> {
    acc: Vec<Vec<M::E>>,
    acc_rev: Vec<Vec<M::E>>,
}
type EIdx = usize;

impl<M: Monoid> MonoidalReroot<M> {
    pub fn new(g: &[Vec<(usize, EIdx)>]) -> Self {
        let n = g.len();
        let mut acc = vec![vec![]; n];
        let mut acc_rev = vec![vec![]; n];
        let mut ch = vec![vec![]; n];
        for i in 0..n {
            acc[i] = vec![M::e(); g[i].len() + 1];
            acc_rev[i] = vec![M::e(); g[i].len() + 1];
        }
        Self::dfs1(0, n, &g, &mut ch);
        for i in 0..n {
            let l = ch[i].len();
            ch[i].sort_unstable_by_key(|&(eidx, _)| eidx);
            for j in 0..l {
                let val = M::add(&acc[i][j], &ch[i][j].1);
                acc[i][j + 1] = val;
            }
            for j in (0..l).rev() {
                let val = M::add(&acc_rev[i][j + 1], &ch[i][j].1);
                acc_rev[i][j] = val;
            }
        }
        Self::dfs2(0, n, &g, &mut ch, &acc, &acc_rev, M::e());
        for i in 0..n {
            let l = ch[i].len();
            ch[i].sort_unstable_by_key(|&(eidx, _)| eidx);
            for j in 0..l {
                let val = M::add(&acc[i][j], &ch[i][j].1);
                acc[i][j + 1] = val;
            }
            for j in (0..l).rev() {
                let val = M::add(&acc_rev[i][j + 1], &ch[i][j].1);
                acc_rev[i][j] = val;
            }
        }
        MonoidalReroot {
            acc: acc,
            acc_rev: acc_rev,
        }
    }
    fn dfs1(
        v: usize, par: usize, g: &[Vec<(usize, EIdx)>],
        ch: &mut [Vec<(EIdx, M::E)>],
    ) -> M::V {
        let mut me = M::e();
        for i in 0..g[v].len() {
            let (w, eidx) = g[v][i];
            if w == par { continue; }
            let chval = Self::dfs1(w, v, g, ch);
            let chval = M::put_edge(&chval, eidx);
            ch[v].push((eidx, chval.clone()));
            me = M::add(&me, &chval);
        }
        M::put_vertex(&me, v)
    }
    fn dfs2(
        v: usize, par: usize, g: &[Vec<(usize, EIdx)>],
        ch: &mut [Vec<(EIdx, M::E)>],
        acc: &[Vec<M::E>],
        acc_rev: &[Vec<M::E>],
        passed: M::E,
    ) {
        let mut parenteidx = None;
        for i in 0..g[v].len() {
            let (w, eidx) = g[v][i];
            if w == par {
                parenteidx = Some(eidx);
                continue;
            }
            let j = ch[v].binary_search_by_key(&eidx, |&(eidx, _)| eidx).unwrap();
            let inherited = M::add(&acc[v][j], &acc_rev[v][j + 1]);
            let inherited = if par >= g.len() {
                inherited
            } else {
                M::add(&inherited, &passed)
            };
            let inherited = M::put_vertex(&inherited, v);
            let inherited = M::put_edge(&inherited, eidx);
            Self::dfs2(w, v, g, ch, acc, acc_rev, inherited);
        }
        if let Some(eidx) = parenteidx {
            ch[v].push((eidx, passed.clone()));
        }
    }
    #[inline]
    pub fn query(&self, v: usize) -> M::V {
        M::put_vertex(&self.acc_rev[v][0], v)
    }
}

enum IndepMonoid {}

impl Monoid for IndepMonoid {
    type V = [i64; 2]; // [その頂点を使わない, 使う]
    type E = [i64; 2]; // [一番上を使わない, 使っても良い]
    fn add(a: &Self::E, b: &Self::E) -> Self::E {
        [a[0] + b[0], a[1] + b[1]]
    }
    fn e() -> Self::E {
        [0, 0]
    }
    fn put_edge(&a: &Self::V, _eidx: EIdx) -> Self::E {
        [a[0], std::cmp::max(a[0], a[1])]
    }
    fn put_vertex(a: &Self::E, _vidx: usize) -> Self::V {
        [a[1], a[0] + 1]
    }
}
