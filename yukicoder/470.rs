#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }


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

/**
 * 2-SAT solver.
 * n: the number of variables (v_1, ..., v_n)
 * cons: constraints, given in 2-cnf
 * i (1 <= i <= n) means v_i, -i (1 <= i <= n) means not v_i.
 * Returns: None if there's no assignment that satisfies cons.
 * Otherwise, it returns an assignment that safisfies cons. (true: true, false: false)
 */
fn two_sat(n: usize, cons: &[(i32, i32)]) -> Option<Vec<bool>> {
    let mut scc = SCC::new(2 * n);
    let ni = n as i32;
    for &(c1, c2) in cons.iter() {
        let x = if c1 > 0 {
            c1 - 1 + ni
        } else {
            -c1 - 1
        } as usize;
        let y = if c2 > 0 {
            c2 - 1
        } else {
            -c2 - 1 + ni
        } as usize;
        scc.add_edge(x, y);
        scc.add_edge((y + n) % (2 * n), (x + n) % (2 * n));
    }
    scc.scc();
    let mut result = vec![false; n];
    let top_ord = scc.top_order();
    for i in 0 .. n {
        if top_ord[i] == top_ord[i + n] {
            return None;
        }
        result[i] = top_ord[i] > top_ord[i + n];
    }
    Some(result)
}

fn main() {
    let n = get();
    let u: Vec<Vec<char>> = (0 .. n).map(|_| get_word().chars().collect()).collect();
    {
        let mut res = HashSet::new();
        for i in 0 .. n {
            res.insert(u[i][0]);
            res.insert(u[i][2]);
            if res.len() < i + 1 {
                println!("Impossible");
	        return;
            }
        }
    }
    assert!(n <= 52);
    let mut pool = Vec::new();
    for i in 0 .. n {
        pool.push((u[i][0], u[i][1 .. 3].to_vec(), i as i32 + 1));
        pool.push((u[i][2], u[i][0 .. 2].to_vec(), - (i as i32 + 1)));
    }
    let mut interfere = Vec::new();
    for i in 0 .. 2 * n {
        for j in 0 .. 2 * n {
            if i == j { continue; }
            let t1 = &pool[i];
            let t2 = &pool[j];
            if t1.0 == t2.0 || t1.1 == t2.1 {
	        interfere.push((-t1.2, -t2.2));
            }
        }
    }
    let res = match two_sat(n, &interfere) {
        None => {
            println!("Impossible");
            return;
        },
        Some(x) => x,
    };
    for i in 0 .. n {
        if res[i] {
            println!("{} {}{}", u[i][0], u[i][1], u[i][2]);
        } else {
            println!("{}{} {}", u[i][0], u[i][1], u[i][2]);
        }
    }
}
