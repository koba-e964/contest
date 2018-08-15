#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }


/// Verified by: https://beta.atcoder.jp/contests/arc045/submissions/3012486
mod tecomp {
    use std::cmp::min;
    const INF: usize = 1 << 28;
    fn dfs(v: usize, par: usize, g: &[Vec<(usize, usize)>],
           ord: &mut [usize], low: &mut [usize], k: &mut usize,
           bridges: &mut Vec<usize>) {
        ord[v] = *k;
        low[v] = *k;
        *k += 1;
        for &(w, eidx) in g[v].iter() {
            if par == w { continue; }
            if ord[w] < ord[v] {
                low[v] = min(low[v], ord[w]);
            } else if ord[w] == INF {
                dfs(w, v, g, ord, low, k, bridges);
                low[v] = min(low[v], low[w]);
                if ord[v] < low[w] {
                    bridges.push(eidx);
                }
            }
        }
    }
    
    fn dfs_comp(v: usize, g: &[Vec<(usize, usize)>],
                ord: &[usize], low: &[usize],
                cur_becomp: usize, becomp_count: &mut usize, becomp: &mut [usize], tree: &mut [Vec<(usize, usize)>], vis: &mut [bool]) {
        becomp[v] = cur_becomp;
        vis[v] = true;
        for &(w, eidx) in g[v].iter() {
            if ord[w] > ord[v] && !vis[w] {
                if ord[v] < low[w] {
                    *becomp_count += 1;
                    tree[cur_becomp].push((*becomp_count, eidx));
                    dfs_comp(w, g, ord, low, *becomp_count, becomp_count, becomp, tree, vis);
                } else {
                    dfs_comp(w, g, ord, low, cur_becomp, becomp_count, becomp, tree, vis);
                }
            }
        }
    }
    
    /// Returns (the number of 2-edge connected components, [the id of the component v belongs to | v <- [0 .. g.len()]], the resulting tree, the ids of bridges).
    /// Graphs are given and provided in the adjacent list format. (to, edge_id).
    /// The provided tree has its own vertex ids, but edge ids are reused.
    pub fn decomp(g: &[Vec<(usize, usize)>])
                 -> (usize, Vec<usize>, Vec<Vec<(usize, usize)>>, Vec<usize>) {
        let n_vert = g.len();
        let mut ord = vec![INF; n_vert];
        let mut low = vec![INF; n_vert];
        let mut k = 0;
        let mut becomp_count = 0;
        let mut becomp = vec![INF; n_vert];
        let mut bridges = Vec::new();
        // rooted forest
        let mut tree = vec![Vec::new(); n_vert];
        let mut vis = vec![false; n_vert];
        for i in 0 .. n_vert {
            if !vis[i] {
                dfs(i, n_vert, &g, &mut ord, &mut low, &mut k, &mut bridges);
                dfs_comp(i, &g, &ord, &low, becomp_count, &mut becomp_count, &mut becomp,
                         &mut tree, &mut vis);
                becomp_count += 1;
            }
        }
        tree = tree[..becomp_count].to_vec();
        (becomp_count, becomp, tree, bridges)
    }
} // mod tecomp

fn dfs_count(v: usize, tree: &[Vec<(usize, usize)>],
             tvis: &mut [bool], conn: &[usize],
             even_edges: &mut HashSet<usize>,
             cur_comp: usize, comp: &mut [usize]) -> usize {
    tvis[v] = true;
    comp[v] = cur_comp;
    let mut cnt = conn[v];
    for &(w, eidx) in tree[v].iter() {
        let sub = dfs_count(w, tree, tvis, conn, even_edges, cur_comp, comp);
        if sub % 2 == 0 {
            even_edges.insert(eidx);
        }
        cnt += sub + 1;
    }
   cnt
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n: usize = get();
    let m = 2 * n + 1;
    let mut x = vec![0; m];
    let mut y = vec![0; m];
    for i in 0 .. m {
        x[i] = get::<usize>() - 1;
        y[i] = get::<usize>() - 1;
    }
    let n_vert = 2 * m;
    let mut g = vec![Vec::new(); n_vert];
    for i in 0 .. m {
        g[x[i]].push((m + y[i], i));
        g[m + y[i]].push((x[i], i));
    }
    let (becomp_count, becomp, tree, bridges) = tecomp::decomp(&g);
    let mut conn = vec![0; becomp_count];
    for i in 0 .. m {
        if becomp[x[i]] == becomp[m + y[i]] {
            conn[becomp[x[i]]] += 1;
        }
    }
    let mut ans = vec![false; m];
    // odd components
    let mut tvis = vec![false; becomp_count];
    let mut odd_comps = Vec::new();
    let mut even_edges = HashSet::new();
    let mut comp = vec![0; becomp_count];
    let mut comp_count = 0;
    for i in 0 .. becomp_count {
        if !tvis[i] {
            let s = dfs_count(i, &tree, &mut tvis, &conn, &mut even_edges, comp_count, &mut comp);
            if s % 2 == 1 {
                odd_comps.push(comp_count);
            }
            comp_count += 1;
        }
    }
    /*
    eprintln!("tree = {:?}", tree);
    eprintln!("conn = {:?}", conn);
    eprintln!("odd_comps = {:?}", odd_comps);
    eprintln!("even_edges = {:?}", even_edges);
    // */
    if odd_comps.len() == 1 {
        let odd_comp = odd_comps[0];
        for i in 0 .. m {
            if comp[becomp[x[i]]] == odd_comp {
                ans[i] = true;
            }
        }
        for b in bridges {
            if comp[becomp[x[b]]] == odd_comp {
                ans[b] = even_edges.contains(&b);
            }
        }
    }
    for i in 0 .. m {
        puts!("{}\n", if ans[i] { "OK" } else { "NG" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// I know this code is wrong.
