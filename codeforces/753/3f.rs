use std::cmp::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
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

// Tags: functional-graphs
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut s = vec![vec![]; n];
        for i in 0..n {
            s[i] = get_word().bytes().collect();
        }
        let mut g = vec![n * m; n * m + 1];
        for i in 0..n {
            for j in 0..m {
                let mut x = i as i32;
                let mut y = j as i32;
                match s[i][j] {
                    b'U' => x -= 1,
                    b'D' => x += 1,
                    b'R' => y += 1,
                    b'L' => y -= 1,
                    _ => panic!(),
                }
                let x = x as usize;
                let y = y as usize;
                if x >= n || y >= m {
                    continue;
                }
                g[i * m + j] = x * m + y;
            }
        }
        let dat = FunGraph::new(&g);
        let mut ma = (0, 0);
        for i in 0..n * m {
            let l = dat.dist_cyc[i] + if dat.to_cyc[i] == n * m {
                0
            } else {
                dat.cyc[dat.to_cyc[i]]
            };
            ma = max(ma, (l, i));
        }
        println!("{} {} {}", ma.1 / m + 1, ma.1 % m + 1, ma.0);
    }
}
