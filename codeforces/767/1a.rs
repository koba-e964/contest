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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: adhoc-data-structure, greedy, constructive
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
    input!(a: [[usize]]);
    for a in a {
        let n = a.len();
        let mut pop = vec![0; n + 1];
        let mut mex = 0;
        for i in 0..n {
            pop[a[i]] += 1;
        }
        while pop[mex] != 0 {
            mex += 1;
        }
        let mut b = vec![];
        let mut pos = 0;
        let mut newmex = mex;
        while pos < n {
            b.push(mex);
            let oldpos = pos;
            let mut rem = mex;
            let mut seen = vec![false; rem];
            while rem > 0 && pos < n {
                let v = a[pos];
                if v < mex && !seen[v] {
                    seen[v] = true;
                    rem -= 1;
                }
                pop[v] -= 1;
                if pop[v] == 0 {
                    newmex = min(newmex, v);
                }
                pos += 1;
            }
            mex = newmex;
            if pos == oldpos {
                pos += 1;
            }
        }
        puts!("{}\n", b.len());
        putvec!(b);
    }
}
