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
    const W: usize = 20_010;
    let mut pr = vec![true; W];
    pr[0] = false;
    pr[1] = false;
    for i in 2..W {
        if !pr[i] { continue; }
        for j in 2..(W - 1) / i + 1 {
            pr[i * j] = false;
        }
    }
    for a in a {
        let n = a.len();
        let s: usize = a.iter().sum();
        if !pr[s] {
            let v: Vec<_> = (1..n + 1).collect();
            puts!("{}\n", n);
            putvec!(v);
            continue;
        }
        let x = a.iter().position(|&x| x % 2 == 1).unwrap();
        let mut v = vec![];
        for i in 0..n {
            if i != x { v.push(i + 1); }
        }
        puts!("{}\n", n - 1);
        putvec!(v);
    }
}
