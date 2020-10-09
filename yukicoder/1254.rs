#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        let (a, b) = ab[i];
        g[a].push((b, i + 1));
        g[b].push((a, i + 1));
    }
    let (_, _, _, br) = tecomp::decomp(&g);
    let br: HashSet<usize> = br.into_iter().collect();
    let mut ans = vec![];
    for i in 1..n + 1 {
        if !br.contains(&i) {
            ans.push(i);
        }
    }
    puts!("{}\n", ans.len());
    putvec!(ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
