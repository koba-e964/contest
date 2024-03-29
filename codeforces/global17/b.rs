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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(a: &[usize], x: usize) -> bool {
    let mut v = vec![];
    for &a in a {
        if a != x {
            v.push(a);
        }
    }
    let mut w = v.clone();
    w.reverse();
    w == v
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(a: [[usize1]]);
    for a in a {
        let n = a.len();
        let mut dif = (0, 0);
        for i in 0..n / 2 {
            if a[i] != a[n - i - 1] {
                dif = (a[i], a[n - i - 1]);
                break;
            }
        }
        if dif == (0, 0) {
            puts!("YES\n");
            continue;
        }
        puts!("{}\n", if calc(&a, dif.0) || calc(&a, dif.1) {
            "YES"
        } else {
            "NO"
        });
    }
}
