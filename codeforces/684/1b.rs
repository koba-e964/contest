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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn calc(n: usize, mut m: usize, k: usize, g: &[Vec<usize>]) -> Result<Vec<usize>, Vec<usize>> {
    let mut deg = vec![0; n];
    let mut que = Vec::new();
    let mut alive = vec![true; n];
    let mut adj = vec![HashSet::new(); n];
    for i in 0..n {
        for &v in &g[i] {
            adj[i].insert(v);
        }
    }
    for i in 0..n {
        deg[i] = g[i].len();
        if deg[i] < k {
            que.push(i);
        }
    }
    let mut rem = n;
    while let Some(x) = que.pop() {
        if !alive[x] {
            continue;
        }
        assert!(deg[x] < k);
        if deg[x] == k - 1 && k < 500 && k * (k - 1) <= 2 * m {
            // check
            let mut ok = true;
            'outer:
            for &v in &adj[x] {
                if !adj[v].contains(&x) {
                    ok = false;
                    break 'outer;
                }
                for &w in &adj[x] {
                    if w == v {
                        continue;
                    }
                    if !adj[v].contains(&w) {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                // found
                let mut ans: Vec<usize> = adj[x].iter().cloned().collect();
                ans.push(x);
                return Err(ans);
            }
        }
        let ad: Vec<_> = adj[x].iter().cloned().collect();
        for &v in &ad {
            adj[v].remove(&x);
            deg[v] -= 1;
            if deg[v] < k {
                que.push(v);
            }
            m -= 1;
        }
        adj[x].clear();
        deg[x] = 0;
        alive[x] = false;
        rem -= 1;
    }
    if rem == 0 {
        Err(vec![])
    } else {
        let ans: Vec<usize> = (0..n).filter(|&i| alive[i]).collect();
        Ok(ans)
    }
}

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
                puts!("{}{}", $v[i] + 1, if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let k: usize = get();
        let mut g = vec![vec![]; n];
        for _ in 0..m {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            g[a].push(b);
            g[b].push(a);
        }
        let ans = calc(n, m, k, &g);
        match ans {
            Ok(e) => {
                puts!("1 {}\n", e.len());
                putvec!(e);
            }
            Err(e) => {
                if e.is_empty() {
                    puts!("-1\n");
                } else {
                    puts!("2\n");
                    putvec!(e);
                }
            }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
