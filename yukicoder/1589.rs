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
        n: usize, k: usize,
        t: usize,
        _c: [[i32; n]; t],
    }
    let ops = n * (n - 1) / 2 * 5 + 2;
    puts!("{}\n", ops);
    for i in 0..n {
        for j in i + 1..n {
            // Whenever we find (0, 1), we replace it with (1, 0).
            puts!("UPD {} 1\n", n);
            puts!("XOR {} {} {}\n", n, n, i);
            puts!("AND {} {} {}\n", n, n, j);
            puts!("XOR {} {} {}\n", i, i, n);
            puts!("XOR {} {} {}\n", j, j, n);
        }
    }
    puts!("UPD {} 0\n", n);
    puts!("XOR {} {} {}\n", n, n, k - 1);
}
