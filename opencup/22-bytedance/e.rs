use std::cmp::*;
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
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
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

fn dfs(v: usize, g: &mut [Vec<(Reverse<usize>, usize)>], edges: &[(usize, usize)]) -> Vec<usize> {
    let mut ret;
    if let Some((Reverse(idx), w)) = g[v].pop() {
        ret = dfs(w, g, edges);
        ret.push(idx);
    } else {
        return vec![];
    }
    let mut ret2;
    if let Some((Reverse(idx), w)) = g[v].pop() {
        ret2 = dfs(w, g, edges);
        ret2.push(idx);
    } else {
        return ret;
    }
    if edges[ret[0]].1 == v {
        ret2.extend_from_slice(&mut ret);
        ret2
    } else {
        ret.extend_from_slice(&mut ret2);
        ret
    }
}

// Tags: euler-circuit
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, b: usize1,
        s: [chars; n],
    }
    let mut edges = vec![(0, 0); n];
    for i in 0..n {
        let s = &s[i];
        let idx = s.iter().position(|&c| c == '1').unwrap();
        if (0..s.len()).any(|j| (s[j] == '1') != (j % m == idx)) {
            puts!("-1\n");
            return;
        }
        edges[i] = (idx, (idx + m - s.len() % m) % m);
    }
    // eprintln!("{:?}", edges);
    let mut g = vec![Vec::new(); m];
    let mut indeg = vec![0; m];
    for i in 0..n {
        let (u, v) = edges[i];
        g[u].push((Reverse(i), v));
        indeg[v] += 1;
    }
    for i in 0..m {
        g[i].sort_unstable();
    }
    let discrep = (0..m).filter(|&v| g[v].len() != indeg[v]).count();
    if discrep >= 3 {
        puts!("-1\n");
        return;
    }
    if discrep == 2 && g[b].len() != indeg[b] + 1 {
        puts!("-1\n");
        return;
    }
    let mut trail = dfs(b, &mut g, &edges);
    if trail.len() != n {
        puts!("-1\n");
        return;
    }
    trail.reverse();
    for v in &mut trail {
        *v += 1;
    }
    putvec!(trail);
}
