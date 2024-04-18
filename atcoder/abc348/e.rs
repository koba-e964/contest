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

fn centroid(v: usize, par: usize, g: &[Vec<usize>], c: &[i64], half: i64) -> Result<i64, usize> {
    let mut sub = c[v];
    let mut ch = vec![];
    for &w in &g[v] {
        if w == par {
            continue;
        }
        let s = centroid(w, v, g, c, half)?;
        sub += s;
        ch.push(s);
    }
    if sub > half {
        return Err(v);
    }
    Ok(sub)
}

fn dfs2(v: usize, par: usize, g: &[Vec<usize>], c: &[i64], dep: i64) -> i64 {
    let mut sub = c[v] * dep;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        sub += dfs2(w, v, g, c, dep + 1);
    }
    sub
}

// Tags: centroid
fn solve() {
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
        c: [i64; n],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let csum: i64 = c.iter().sum();
    let centroid = centroid(0, n, &g, &c, csum / 2).unwrap_err();
    println!("{}", dfs2(centroid, n, &g, &c, 0));
}
