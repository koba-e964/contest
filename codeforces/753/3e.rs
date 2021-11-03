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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        nms: [(i64, i64, chars); t],
    }
    for (n, m, s) in nms {
        let mut lox = 0;
        let mut loy = 0;
        let mut hix = 0;
        let mut hiy = 0;
        let mut x = 0;
        let mut y = 0;
        let mut px = -1;
        let mut py = -1;
        let mut ok = true;
        for c in s {
            match c {
                'D' => x += 1,
                'U' => x -= 1,
                'R' => y += 1,
                'L' => y -= 1,
                _ => panic!(),
            }
            px = -lox + 1;
            py = -loy + 1;
            lox = min(lox, x);
            loy = min(loy, y);
            hix = max(hix, x);
            hiy = max(hiy, y);
            if hix - lox >= n || hiy - loy >= m {
                ok = false;
                break;
            }
        }
        if ok {
            px = -lox + 1;
            py = -loy + 1;
        }
        puts!("{} {}\n", px, py);
    }
}
