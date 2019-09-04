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
        n: usize,
    }
    let h = 13;
    let w = 32;
    let mut ans = vec![vec!['#'; w]; h];

    ans[3][1] = 'S';
    ans[5][1] = 'G';
    for i in 0..7 {
        for j in 0..3 {
            for k in 0..3 {
                if j != 1 || k != 1 {
                    ans[j + 1][4 * i + k + 2] = '.';
                    ans[j + 9][4 * i + k + 2] = '.';
                }
            }
        }
        ans[3][4 * i + 5] = '.';
        ans[9][4 * i + 5] = '.';
    }
    for i in 2..29 {
        ans[5][i] = '.';
        ans[7][i] = '.';
    }
    ans[6][28] = '.';
    for j in 3..10 {
        ans[j][30] = '.';
    }

    let mut holepos = vec![];
    for i in 0..7 {
        holepos.push((4, 1 + 4 * i));
    }
    holepos.push((5, 29));
    for i in (0..7).rev() {
        holepos.push((8, 1 + 4 * i));
    }

    for i in 0..11 {
        if (n & 1 << i) != 0 {
            let (x, y) = holepos[i];
            ans[x][y] = '.';
        }
    }

    puts!("{} {}\n", h, w);
    for i in 0..h {
        for j in 0..w {
            puts!("{}", ans[i][j]);
        }
        puts!("\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
