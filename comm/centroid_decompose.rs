fn centroid_decompose(g: &[Vec<(usize, i64)>]) -> Vec<(usize, usize, i64)> {
    fn find_subtree_sizes(g: &[Vec<(usize, i64)>], v: usize, par: usize,
                          dp: &mut [usize], vis: &[bool]) {
        let mut sum = 1;
        for &(w, _) in &g[v] {
            if par == w || vis[w] { continue; }
            find_subtree_sizes(g, w, v, dp, vis);
            sum += dp[w];
        }
        dp[v] = sum;
    }
    fn centroid_decompose_inner(g: &[Vec<(usize, i64)>], v: usize, par: usize,
                                cost: i64, edges: &mut Vec<(usize, usize, i64)>,
                                dp: &mut [usize], vis: &mut [bool]) {
        let n = g.len();
        find_subtree_sizes(g, v, n, dp, vis);
        let (cent, dist) = {
            let sz = dp[v];
            let find_centroid = |mut v: usize, mut par: usize| {
                let mut dist = 0;
                loop {
                    let mut has_majority = false;
                    for &(w, c) in &g[v] {
                        if par == w || vis[w] { continue; }
                        if dp[w] > sz / 2 {
                            dist += c;
                            par = v;
                            v = w;
                            has_majority = true;
                            break;
                        }
                    }
                    if !has_majority {
                        return (v, dist);
                    }
                }
            };
            find_centroid(v, n)
        };
        let g_cent = g[cent].clone();
        if par < n {
            edges.push((par, cent, dist + cost));
        }
        // v was selected as a centroid
        // and will be ignored in the following decomposition procedure
        vis[cent] = true;
        for &(w, c) in &g_cent {
            if !vis[w] {
                centroid_decompose_inner(g, w, cent, c, edges, dp, vis);
            }
        }
    }
    let n = g.len();
    let mut edges = vec![];
    // This Vec is reused many times
    let mut dp = vec![0; n];
    let mut vis = vec![false; n];
    centroid_decompose_inner(&g, 0, n, 0, &mut edges, &mut dp, &mut vis);
    edges
}
