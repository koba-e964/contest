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
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

#[derive(Default, Clone, Debug)]
struct MexLeaveOne {
    mex1: usize,
    mex2: usize,
    f: Vec<usize>,
}

impl MexLeaveOne {
    fn leave_one(&self, excl: usize) -> usize {
        if excl >= self.mex1 {
            return self.mex1;
        }
        if self.f[excl] == 1 {
            return excl;
        }
        self.mex1
    }
    fn exchange_one(&self, excl: usize, incl: usize) -> usize {
        if excl == incl {
            return self.mex1;
        }
        let mex = self.add_one(incl);
        if excl >= mex {
            return mex;
        }
        let res = if self.f[excl] == 1 {
            excl
        } else {
            mex
        };
        res
    }
    fn add_one(&self, incl: usize) -> usize {
        if incl == self.mex1 {
            self.mex2
        } else {
            self.mex1
        }
    }
    fn as_is(&self) -> usize {
        self.mex1
    }
}

fn dfs1(
    v: usize, par: usize, g: &[Vec<usize>],
    dp_loo: &mut [MexLeaveOne], dp2: &mut [Vec<usize>],
) {
    let mut seen = std::collections::HashSet::new();
    let mut mydp2 = vec![0; g[v].len()];
    for i in 0..g[v].len() {
        let w = g[v][i];
        if w == par { continue; }
        dfs1(w, v, g, dp_loo, dp2);
        mydp2[i] = dp_loo[w].as_is();
        seen.insert(mydp2[i]);
    }
    let mut mex1 = 0;
    while seen.contains(&mex1) {
        mex1 += 1;
    }
    let mut mex2 = mex1 + 1;
    while seen.contains(&mex2) {
        mex2 += 1;
    }
    let mut f = vec![0; mex2];
    for i in 0..g[v].len() {
        let w = g[v][i];
        if w == par { continue; }
        if mydp2[i] < mex2 {
            f[mydp2[i]] += 1;
        }
    }
    let loo = MexLeaveOne {
        mex1: mex1,
        mex2: mex2,
        f: f,
    };
    dp_loo[v] = loo;
    dp2[v] = mydp2;
}
fn dfs2(
    v: usize, par: usize, g: &[Vec<usize>],
    dp1: &mut [usize],
    dp_loo: &[MexLeaveOne],
    dp2: &mut [Vec<usize>],
    passed: usize,
) {
    for i in 0..g[v].len() {
        let w = g[v][i];
        if w == par {
            dp2[v][i] = passed;
            continue;
        }
        let inherited = if par >= g.len() {
            dp_loo[v].leave_one(dp2[v][i])
        } else {
            dp_loo[v].exchange_one(dp2[v][i], passed)
        };
        dfs2(w, v, g, dp1, dp_loo, dp2, inherited);
    }
    dp1[v] = if par >= g.len() {
        dp_loo[v].as_is()
    } else {
        dp_loo[v].add_one(passed)
    };
}

// Tags: rerooting
fn solve() {
    input! {
        n: usize, m: usize,
        a: [usize1; m],
        uv: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut dp1 = vec![0; n];
    let mut dp2 = vec![vec![]; n];
    let mut dp_loo = vec![MexLeaveOne::default(); n];
    dfs1(0, n, &g, &mut dp_loo, &mut dp2);
    dfs2(0, n, &g, &mut dp1, &dp_loo, &mut dp2, 0);
    let mut gr = 0;
    for &a in &a {
        gr ^= dp1[a];
    }
    if gr == 0 {
        println!("-1 -1");
        return;
    }
    let mut pop = vec![false; n];
    for &a in &a {
        pop[a] = true;
    }
    for i in 0..n {
        if !pop[i] { continue; }
        for j in 0..g[i].len() {
            if (gr ^ dp1[i] ^ dp2[i][j]) == 0 {
                let idx = a.iter().position(|&a| a == i).unwrap();
                println!("{} {}", idx + 1, g[i][j] + 1);
                return;
            }
        }
    }
}
