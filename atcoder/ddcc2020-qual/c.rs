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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize, k: usize,
        s: [chars; h],
    }
    let mut ans = vec![vec![0; w]; h];
    let mut used = 0;
    for i in 0..h {
        let mut st = vec![];
        for j in 0..w {
            if s[i][j] == '#' {
                st.push(j);
            }
        }
        if st.is_empty() {
            continue;
        }
        for j in 0..st.len() {
            let lo = if j == 0 { 0 } else { st[j - 1] + 1 };
            used += 1;
            for k in lo..st[j] + 1 {
                ans[i][k] = used;
            }
        }
        for k in st[st.len() - 1]..w {
            ans[i][k] = used;
        }
    }
    for i in 1..h {
        for j in 0..w {
            if ans[i][j] == 0 {
                ans[i][j] = ans[i - 1][j];
            }
        }
    }
    for i in (0..h - 1).rev() {
        for j in 0..w {
            if ans[i][j] == 0 {
                ans[i][j] = ans[i + 1][j];
            }
        }
    }
    assert_eq!(k, used);
    for i in 0..h {
        for j in 0..w {
            puts!("{}{}", ans[i][j], if j + 1 == w { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
