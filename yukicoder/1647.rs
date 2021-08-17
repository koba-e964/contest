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

fn dfs(v: usize, par: usize, vis: &mut [bool], g: &[Vec<(usize, usize)>],
) -> Result<(), (bool, usize, Vec<usize>)> {
    if vis[v] {
        return Err((true, v, vec![]));
    }
    vis[v] = true;
    for &(w, idx) in &g[v] {
        if w == par {
            continue;
        }
        let sub = dfs(w, v, vis, g);
        if let Err((extending, orig, mut sub)) = sub {
            if extending {
                sub.push(idx + 1);
            }
            return Err((extending && orig != v, orig, sub));
        }
    }
    Ok(())
}

// Tags: dfs, cycle-detection, off-by-one-error, index-manipulation
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
        h: usize, w: usize, n: usize,
        xy: [(usize1, usize1); n],
    }
    let mut g = vec![vec![]; h + w];
    for i in 0..n {
        let (x, y) = xy[i];
        g[x].push((h + y, i));
        g[h + y].push((x, i));
    }
    let mut vis = vec![false; h + w];
    for i in 0..h {
        if vis[i] {
            continue;
        }
        let res = dfs(i, h + w, &mut vis, &g);
        if let Err((_, _, mut path)) = res {
            path.reverse();
            if xy[path[0] - 1].1 != xy[path[1] - 1].1 {
                let u = path.pop().unwrap();
                path.insert(0, u);
            }
            puts!("{}\n", path.len());
            putvec!(path);
            return;
        }
    }
    puts!("-1\n");
}
