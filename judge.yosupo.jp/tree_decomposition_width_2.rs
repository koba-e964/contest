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

// https://ei1333.hateblo.jp/entry/2020/02/12/150319
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
struct BagIdx(usize);
#[derive(PartialEq, Eq, Clone, Debug)]
struct TDNode {
    bag: Vec<usize>,
    children: Vec<BagIdx>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum State {
    Unfilled,
    Picked,
    Hanging(BagIdx),
}

fn decomp(mut g: Vec<Vec<usize>>) -> Option<Vec<TDNode>> {
    let n = g.len();
    let mut gset = vec![HashSet::<usize>::new(); n];
    let mut deg = vec![0; n];
    for i in 0..n {
        for &j in &g[i] {
            deg[i] += 1;
            if i < j {
                gset[i].insert(j);
            }
        }
    }
    let mut que = VecDeque::new();
    let mut used = vec![State::Unfilled; n];
    let mut ret = vec![];
    for i in 0..n {
        if deg[i] <= 2 {
            que.push_back(i);
        }
    }
    while let Some(idx) = que.pop_front() {
        if deg[idx] > 2 || used[idx] != State::Unfilled {
            continue;
        }
        let mut bag = vec![idx];
        let mut children = vec![];
        let mut u = None::<usize>;
        let mut v = None::<usize>;
        for &to in &g[idx] {
            if used[to] == State::Unfilled {
                if u == None {
                    u = Some(to);
                } else {
                    v = Some(to);
                }
                bag.push(to);
            } else if let State::Hanging(bagidx) = used[to] {
                children.push(bagidx);
                used[to] = State::Picked;
            }
        }
        if let Some(u) = u {
            if v == None {
                deg[u] -= 1;
            } else {
                let v = v.unwrap();
                let (u, v) = if u > v {
                    (v, u)
                } else {
                    (u, v)
                };
                if gset[u].contains(&v) {
                    deg[u] -= 1;
                    deg[v] -= 1;
                } else {
                    g[u].push(v);
                    g[v].push(u);
                    gset[u].insert(v);
                }
            }
        }
        for &to in &g[idx] {
            if deg[to] <= 2 {
                que.push_back(to);
            }
        }
        used[idx] = State::Hanging(BagIdx(ret.len()));
        deg[idx] = 0;
        ret.push(TDNode {
            bag: bag,
            children: children,
        });
    }
    for i in 0..n {
        if deg[i] > 0 {
            return None;
        }
    }
    let root = ret.len() - 1;
    for i in 0..n {
        if let State::Hanging(BagIdx(v)) = used[i] {
            if v != root {
                ret[root].children.push(BagIdx(v));
            }
        }
    }
    Some(ret)
}

// Tags: tree-decomposition
fn main() {
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
        _p: chars, _tw: chars, n: usize, m: usize,
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let ans = decomp(g);
    let ans = if let Some(ans) = ans {
        ans
    } else {
        puts!("-1\n");
        return;
    };
    puts!("s td {} {} {}\n", ans.len(), 2, n);
    for i in 0..ans.len() {
        puts!("b {}", i + 1);
        for &v in &ans[i].bag {
            puts!(" {}", v + 1);
        }
        puts!("\n");
    }
    for i in 0..ans.len() {
        for &v in &ans[i].children {
            puts!("{} {}\n", i + 1, v.0 + 1);
        }
    }
}
