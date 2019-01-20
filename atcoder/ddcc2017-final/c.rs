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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs(m: usize, g: &[Vec<(usize, i64, usize)>],
       pot: &mut [Option<(i64, Vec<usize>)>],
       v: usize, cur: i64,
       trace: &mut Vec<usize>)
       -> Result<(), (i64, Vec<usize>)> {
    if let Some((pot, ref orig_trace)) = pot[v] {
        if cur != pot {
            // TODO
            for &eidx in orig_trace {
                trace.push((eidx + m) % (2 * m));
            }
            return Err((cur - pot, trace.clone()));
        }
        return Ok(())
    }
    pot[v] = Some((cur, trace.clone()));
    for &(w, cost, idx) in g[v].iter() {
        trace.push(idx);
        dfs(m, g, pot, w, cur + cost, trace)?;
        trace.pop().unwrap();
    }
    Ok(())
}


fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        m: usize,
        abc: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    let mut tbl = vec![(0, 0, 0); 2 * m];
    for (i, (a, b, c)) in abc.into_iter().enumerate() {
        g[a].push((b, c, i));
        tbl[i] = (a, g[a].len() - 1, c);
        g[b].push((a, -c, m + i));
        tbl[i + m] = (b, g[b].len() - 1, -c);
    }
    let mut pot = vec![None; n];
    match dfs(m, &g, &mut pot, 0, 0, &mut vec![]) {
        Ok(()) => {
            puts!("Yes\n");
        },
        Err((diff, trace)) => {
            for eidx in trace {
                // adjust
                let (a, b, c) = tbl[eidx];
                g[a][b].1 = c - diff;
                let (a, b, c) = tbl[(eidx + m) % (2 * m)];
                g[a][b].1 = c + diff;
                // Check
                let mut pot = vec![None; n];
                if Ok(()) == dfs(m, &g, &mut pot, 0, 0, &mut Vec::new()) {
                    puts!("Yes\n");
                    return
                }
                // recover
                let (a, b, c) = tbl[eidx];
                g[a][b].1 = c;
                let (a, b, c) = tbl[(eidx + m) % (2 * m)];
                g[a][b].1 = c;
            }
            puts!("No\n");
        },
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
