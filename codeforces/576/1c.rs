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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        graphs: [(usize, [(usize1, usize1)])],
    }
    for (n, edges) in graphs {
        let mut g = vec![vec![]; 3 * n];
        for (i, (a, b)) in edges.into_iter().enumerate() {
            g[a].push((b, i));
            g[b].push((a, i));
        }
        let mut rmed = vec![false; 3 * n];
        let mut mat = vec![];
        let mut ind = vec![];
        for i in 0..3 * n {
            if rmed[i] { continue; }
            let mut pick = None;
            for &(w, idx) in &g[i] {
                if !rmed[w] {
                    pick = Some((w, idx));
                    break;
                }
            }
            match pick {
                Some((w, idx)) => {
                    mat.push(idx);
                    rmed[w] = true;
                }
                None => ind.push(i),
            }
            rmed[i] = true;
        }
        if mat.len() >= n {
            puts!("Matching\n");
            for i in 0..n {
                puts!("{}{}", mat[i] + 1, if i + 1 == n { "\n" } else { " " });
            }
        } else {
            assert!(ind.len() >= n);
            puts!("IndSet\n");
            for i in 0..n {
                puts!("{}{}", ind[i] + 1, if i + 1 == n { "\n" } else { " " });
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
