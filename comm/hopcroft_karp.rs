// https://en.wikipedia.org/wiki/Hopcroft–Karp_algorithm#Pseudocode
fn hopcroft_karp(g: &[Vec<usize>], m: usize)
                 -> (usize, Vec<usize>) {
    let n = g.len();
    let mut pu = vec![m; n + 1]; // n: NIL
    let mut pv = vec![n; m + 1]; // m: NIL
    let mut dist = vec![0; n + 1];

    let mut matching = 0;

    fn bfs(g: &[Vec<usize>], pu: &mut [usize], pv: &mut [usize],
           dist: &mut [usize]) -> bool {
        let n = g.len();
        let m = pv.len() - 1;
        let mut que = std::collections::VecDeque::new();
        for i in 0..n {
            if pu[i] == m {
                dist[i] = 0;
                que.push_back(i);
            } else {
                dist[i] = usize::max_value();
            }
        }
        dist[n] = usize::max_value();
        while let Some(u) = que.pop_front() {
            if dist[u] < dist[n] {
                for &v in &g[u] {
                    if dist[pv[v]] == usize::max_value() {
                        dist[pv[v]] = dist[u] + 1;
                        que.push_back(pv[v]);
                    }
                }
            }
        }
        dist[n] != usize::max_value()
    }

    fn dfs(g: &[Vec<usize>], pu: &mut [usize], pv: &mut [usize],
           dist: &mut [usize],
           u: usize) -> bool {
        let n = g.len();
        if u != n {
            for &v in &g[u] {
                if dist[pv[v]] == dist[u] + 1 {
                    let pvv = pv[v];
                    if dfs(g, pu, pv, dist, pvv) {
                        pv[v] = u;
                        pu[u] = v;
                        return true;
                    }
                }
            }
            dist[u] = usize::max_value();
            return false;
        }
        true
    }

    while bfs(g, &mut pu, &mut pv, &mut dist) {
        for u in 0..n {
            if pu[u] == m {
                if dfs(g, &mut pu, &mut pv, &mut dist, u) {
                    matching += 1;
                }
            }
        }
    }
    (matching, pu)
}
