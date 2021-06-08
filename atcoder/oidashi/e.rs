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

// Tags: subsequence-dp, lexicographical-minimum, greedy
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        d: [usize; n],
    }
    let mut nxt = vec![[n; 10]; n + 2];
    for i in (0..n).rev() {
        nxt[i] = nxt[i + 1];
        nxt[i][d[i]] = i;
    }
    let mut dp0 = vec![0; n + 1];
    for i in (0..n).rev() {
        let mut mi = 1 << 28;
        for j in 0..10 {
            let nx = nxt[i + 2][j];
            if nx < n {
                mi = min(mi, dp0[nx] + 1);
            } else {
                mi = 0;
            }
        }
        dp0[i] = mi;
    }
    let mut l = 1 << 28;
    for i in 1..10 {
        let nxt = nxt[0][i];
        if nxt < n {
            l = min(l, dp0[nxt] + 1);
        } else {
            l = 0;
        }
    }
    // The range 1 <= x <= 10^l - 1 is fully covered.
    let mut path = vec![0; l + 1];
    let mut cur = 0;
    let mut pos = 0;
    while pos < l + 1 {
        let mut dig = 10;
        for i in if pos == 0 { 1 } else { 0 }..10 {
            let nxt = nxt[cur][i];
            if nxt >= n || dp0[nxt] < l - pos {
                dig = i;
                break;
            }
        }
        assert!(dig < 10);
        path[pos] = dig;
        cur = nxt[cur][dig] + 2;
        pos += 1;
    }
    for i in 0..l + 1 {
        puts!("{}", path[i]);
    }
    puts!("\n");
}
