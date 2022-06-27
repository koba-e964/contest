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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        s: chars,
        w: [i64; n],
    }
    let mut coo = w.clone();
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut b = vec![0; m];
    let mut c = vec![0; m];
    for i in 0..n {
        let idx = coo.binary_search(&w[i]).unwrap();
        if s[i] == '1' {
            c[idx] += 1;
        } else {
            b[idx] += 1;
        }
    }
    let mut acc_b = vec![0; m + 1];
    let mut acc_c = vec![0; m + 1];
    for i in 0..m {
        acc_b[i + 1] = acc_b[i] + b[i];
        acc_c[i + 1] = acc_c[i] + c[i];
    }
    let mut ma = 0;
    for i in 0..m + 1 {
        let tmp = acc_b[i] + (acc_c[m] - acc_c[i]);
        ma = max(ma, tmp);
    }
    println!("{}", ma);
}
