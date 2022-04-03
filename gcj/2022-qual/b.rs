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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
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
        t: usize,
        col: [[[i64; 4]; 3]; t],
    }
    const INF: i64 = 1 << 50;
    for i in 0..t {
        puts!("Case #{}: ", i + 1);
        let mut mi = vec![INF; 4];
        for j in 0..3 {
            for k in 0..4 {
                mi[k] = min(mi[k], col[i][j][k]);
            }
        }
        let s: i64 = mi.iter().sum();
        if s < 1_000_000 {
            puts!("IMPOSSIBLE\n");
            continue;
        }
        let mut ans = vec![0; 4];
        let mut rem = 1_000_000;
        for j in 0..4 {
            let x = min(rem, mi[j]);
            ans[j] = x;
            rem -= x;
        }
        assert_eq!(rem, 0);
        putvec!(ans);
    }
}
