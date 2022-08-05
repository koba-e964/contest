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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(
    p: &[usize],
    q: &[usize],
    invp: &[usize],
    invq: &[usize],
    pr: core::ops::Range<usize>,
    qr: core::ops::Range<usize>,
    ans: &mut [(usize, usize)],
) -> Option<usize> {
    if pr.start == pr.end {
        return Some(0);
    }
    let r = p[pr.start];
    let idx = invq[r];
    if idx < qr.start || qr.end <= idx {
        return None;
    }
    let len1 = idx - qr.start;
    let c1 = dfs(p, q, invp, invq, pr.start + 1..pr.start + len1 + 1, qr.start..idx, ans)?;
    let c2 = dfs(p, q, invp, invq, pr.start + len1 + 1..pr.end, idx + 1..qr.end, ans)?;
    ans[r] = (c1, c2);
    Some(r + 1)
}

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
        n: usize,
        p: [usize1; n],
        q: [usize1; n],
    }
    if p[0] != 0 {
        puts!("-1\n");
        return;
    }
    let mut invp = vec![0; n];
    let mut invq = vec![0; n];
    for i in 0..n {
        invp[p[i]] = i;
        invq[q[i]] = i;
    }
    let mut ans = vec![(0, 0); n];
    if dfs(&p, &q, &invp, &invq, 0..n, 0..n, &mut ans).is_some() {
        for i in 0..n {
            let (x, y) = ans[i];
            puts!("{} {}\n", x, y);
        }
    } else {
        puts!("-1\n");
    }
}
