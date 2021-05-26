// Split a namori graph into a cycle and forests.
// Verified by: https://atcoder.jp/contests/jsc2021/submissions/22926985
struct NamoriSplit<T> {
    #[allow(unused)]
    // The graph without the only cycle.
    forest: Vec<Vec<(usize, T)>>,
    #[allow(unused)]
    // The graph without the only cycle, with metadata dropped.
    forest_e: Vec<Vec<usize>>,
    #[allow(unused)]
    // to_root[i] = r <=> i belongs to the tree containing r, r is in the cycle
    to_root: Vec<usize>,
    #[allow(unused)]
    // The vertices in the cycle.
    // roots[i] and roots[i + 1] are connected in the cycle.
    roots: Vec<usize>,
    #[allow(unused)]
    // Sequence of metadata.
    cycle: Vec<T>,
    #[allow(unused)]
    // vertex |--> the index of its root in roots
    to_id: Vec<usize>,
}

fn namori_split<T: Clone>(g: &[Vec<(usize, T)>]) -> NamoriSplit<T> {
    fn dfs1(v: usize, par: usize, g: &[Vec<usize>], r: usize, root: &mut [usize]) {
        root[v] = r;
        for &w in &g[v] {
            if w == par {
                continue;
            }
            dfs1(w, v, g, r, root);
        }
    }
    fn get_root_seq<T: Clone>(roots: &[usize], cyc: &[Vec<(usize, T)>]) -> Vec<(usize, T)> {
        let mut root_seq = vec![];
        let v = roots[0];
        if roots.len() == 2 {
            for i in 0..2 {
                root_seq.push((roots[i], cyc[v][i].1.clone()));
            }
            return root_seq;
        }
        let (mut w, c) = cyc[v][0].clone();
        root_seq.push((v, c));
        while w != v {
            let mut nxt = None;
            for &(a, ref b) in &cyc[w] {
                if a == root_seq[root_seq.len() - 1].0 {
                    continue;
                }
                nxt = Some((a, b.clone()));
                break;
            }
            let nxt = nxt.unwrap();
            root_seq.push((w, nxt.1));
            w = nxt.0;
        }
        root_seq
    }
    let n = g.len();
    let mut deg = vec![0; n];
    for i in 0..n {
        deg[i] = g[i].len();
    }
    let mut que = vec![];
    for i in 0..n {
        if deg[i] == 1 {
            que.push(i);
        }
    }
    let mut rem = vec![true; n];
    while let Some(v) = que.pop() {
        if !rem[v] {
            continue;
        }
        rem[v] = false;
         for &(w, _) in &g[v] {
            if rem[w] {
                deg[w] -= 1;
                if deg[w] == 1 {
                    que.push(w);
                }
            }
        }
    }
    let mut forest = vec![vec![]; n];
    let mut forest_e = vec![vec![]; n];
    let mut cyc = vec![vec![]; n];
    let mut roots = vec![];
    for i in 0..n {
        if rem[i] {
            roots.push(i);
        }
        for &(a, ref c) in &g[i] {
            if rem[i] && rem[a] {
                cyc[i].push((a, c.clone()));
            } else {
                forest[i].push((a, c.clone()));
                forest_e[i].push(a);
            }
        }
    }
    let mut to_root = vec![0; n];
    for &r in &roots {
        dfs1(r, n, &forest_e, r, &mut to_root);
    }
    let root_seq;
    let mut to_id = vec![n; n];
    let mut roots_0 = vec![];
    let mut cycle = vec![];
    {
        root_seq = get_root_seq(&roots, &cyc);
        for i in 0..root_seq.len() {
            to_id[root_seq[i].0] = i;
            roots_0.push(root_seq[i].0);
            cycle.push(root_seq[i].1.clone());
        }
    }
    for i in 0..n {
        to_id[i] = to_id[to_root[i]];
    }
    NamoriSplit {
        forest: forest,
        forest_e: forest_e,
        to_root: to_root,
        roots: roots_0,
        cycle: cycle,
        to_id: to_id,
    }
}
