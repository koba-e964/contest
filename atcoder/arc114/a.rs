use std::cmp::*;
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
    input! {
        n: usize,
        a: [i64; n],
    }
    let pr = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut mi = 1i64;
    for &p in &pr {
        mi *= p;
    }
    let mut set = vec![0; n];
    for i in 0..n {
        for j in 0..15 {
            if a[i] % pr[j] == 0 {
                set[i] |= 1 << j;
            }
        }
    }
    for bits in 0..1 << 15 {
        if set.iter().all(|&s| (s & bits) != 0) {
            let mut tmp = 1;
            for i in 0..15 { if (bits & 1 << i) != 0 { tmp *= pr[i]; }}
            mi = min(mi, tmp);
        }
    }
    println!("{}", mi);
}
