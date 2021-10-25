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
        n: usize, x: i64, y: i64,
        a: [i64; n],
    }
    let mut y = y;
    let mut dc = 0;
    let mut p = vec![1; n];
    for i in (0..n).rev() {
        let xm = a[i] & !dc;
        if y == xm {
            p[i] = 3;
            y = 0;
            dc = -1;
            break;
        }
        if (y & xm) == xm {
            dc |= xm;
            y ^= xm;
            p[i] = 2;
            continue;
        }
        if (y & xm) == y {
            dc |= !xm;
            p[i] = 1;
            continue;
        }
        puts!("-1\n");
        return;
    }
    if (x & !dc) == y {
        putvec!(p);
    } else {
        puts!("-1\n");
    }
}
