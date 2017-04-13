#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
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

/**
 * Strong connected components.
 * Verified by: yukicoder No.470 (http://yukicoder.me/submissions/145785)
 */
struct SCC {
    n: usize,
    ncc: usize,
    g: Vec<Vec<usize>>, // graph in adjacent list
    rg: Vec<Vec<usize>>, // reverse graph
    cmp: Vec<usize>, // topological order
}

impl SCC {
    fn new(n: usize) -> Self {
        SCC {
            n: n,
            ncc: n + 1,
            g: vec![Vec::new(); n],
            rg: vec![Vec::new(); n],
            cmp: vec![0; n],
        }
    }
    fn add_edge(&mut self, from: usize, to: usize) {
        self.g[from].push(to);
        self.rg[to].push(from);
    }
    fn dfs(&self, v: usize, used: &mut [bool], vs: &mut Vec<usize>) {
        used[v] = true;
        for &w in self.g[v].iter() {
            if !used[w] {
               self.dfs(w, used, vs);
            }
        }
        vs.push(v);
    }
    fn rdfs(&self, v: usize, k: usize,
            used: &mut [bool], cmp: &mut [usize]) {
        used[v] = true;
        cmp[v] = k;
        for &w in self.rg[v].iter() {
            if !used[w] {
                self.rdfs(w, k, used, cmp);
            }
        }
    }
    fn scc(&mut self) -> usize {
        let n = self.n;
        let mut used = vec![false; n];
        let mut vs = Vec::new();
        let mut cmp = vec![0; n];
        for v in 0 .. n {
            if !used[v] { self.dfs(v, &mut used, &mut vs); }
        }
        for u in used.iter_mut() {
            *u = false;
        }
        let mut k = 0;
        for &t in vs.iter().rev() {
            if !used[t] { self.rdfs(t, k, &mut used, &mut cmp); k += 1; }
        }
        self.ncc = k;
        self.cmp = cmp;
        k
    }
    #[allow(dead_code)]
    fn top_order(&self) -> Vec<usize> {
        assert!(self.ncc <= self.n);
        self.cmp.clone()
    }
    /*
     * Returns a dag whose vertices are scc's, and whose edges are those of the original graph.
     */
    #[allow(dead_code)]
    fn dag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![HashSet::new(); ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[i]].insert(self.cmp[to]);
                }
            }
        }
        ret.into_iter().map(|set| set.into_iter().collect()).collect()
    }
    #[allow(dead_code)]
    fn rdag(&self) -> Vec<Vec<usize>> {
        assert!(self.ncc <= self.n);
        let ncc = self.ncc;
        let mut ret = vec![HashSet::new(); ncc];
        let n = self.n;
        for i in 0 .. n {
            for &to in self.g[i].iter() {
                if self.cmp[i] != self.cmp[to] {
                    assert!(self.cmp[i] < self.cmp[to]);
                    ret[self.cmp[to]].insert(self.cmp[i]);
                }
            }
        }
        ret.into_iter().map(|set| set.into_iter().collect()).collect()
    }
}


fn calc(f: &[usize]) -> usize {
    let n = f.len();
    let mut two = 0;
    let mut maxcy = 0;
    let mut scc = SCC::new(n);
    for i in 0 .. n {
        scc.add_edge(i, f[i]);
    }
    // Find the maximum cycle
    let ncc = scc.scc();
    let top_ord = scc.top_order();
    let dag = scc.dag();
    let mut cycles = Vec::new();
    let mut conn = vec![Vec::new(); ncc];
    for i in 0 .. n {
        conn[top_ord[i]].push(i);
    }
    for i in 0 .. ncc {
        if dag[i].len() == 0 { // cycle
            let c = conn[i].len();
            maxcy = max(maxcy, c);
            if c == 2 {
                two += 1;
            }
            cycles.push((i, c));
        }
    }
    let rdag = scc.rdag();
    let mut two_chains_max = 0;
    for (dag_v, clen) in cycles {
        if clen != 2 { continue; }
        let mut tbl = HashMap::new();
        for &dag_w in rdag[dag_v].iter() {
            let w = conn[dag_w][0];
            let orig = f[w];
            let mut chainlen = 1;
            let mut que = VecDeque::new();
            que.push_back((dag_w, 1));
            while let Some((dag_x, dist)) = que.pop_front() {
                chainlen = max(chainlen, dist);
                for &dag_nxt in rdag[dag_x].iter() {
                    que.push_back((dag_nxt, dist + 1));
                }
            }
            if !tbl.contains_key(&orig) {
                tbl.insert(orig, chainlen);
            } else {
                let tmp = tbl.get(&orig).cloned().unwrap();
                tbl.insert(orig, max(tmp, chainlen));
            }
        }
        let mut tot = 0;
        for (_, f) in tbl {
            tot += f;
        }
        two_chains_max += tot;
    }
    let mut two_max = 2 * two;
    two_max += two_chains_max;
    max(maxcy, two_max)
}

fn solve() {
    let t: usize = get();
    for case_nr in 1 .. t + 1 {
        let n = get();
        let f: Vec<usize> = (0 .. n).map(|_| get::<usize>() - 1).collect();
        println!("Case #{}: {}", case_nr, calc(&f));
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
