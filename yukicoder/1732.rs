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
    input! {
        t: usize,
        n: [usize; t],
    }
    let s = "ACCACCCCCCCCABACAAFFE";
    for n in n {
        if n % 21 == 5 {
            puts!("ABDCC");
            for _ in 0..n / 21 {
                puts!("{}", s);
            }
            puts!("\n");
            continue;
        }
        if n % 21 == 10 {
            puts!("AAABCCCDFC");
            for _ in 0..n / 21 {
                puts!("{}", s);
            }
            puts!("\n");
            continue;
        }
        if n % 21 == 0 {
            for _ in 0..n / 21 {
                puts!("{}", s);
            }
            puts!("\n");
            continue;
        }
        puts!("-1\n");
    }
}
