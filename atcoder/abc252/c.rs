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
        s: [chars; n],
    }
    let mut tbl = vec![vec![0; 10]; 10];
    for i in 0..n {
        for j in 0..10 {
            let idx = s[i][j] as u8 - b'0';
            tbl[idx as usize][j] += 1;
        }
    }
    let mut mi = 1 << 28;
    for i in 0..10 {
        let mut tmp = 0;
        for j in 0..10 {
            if tbl[i][j] > 0 {
                tmp = max(tmp, 10 * tbl[i][j] - 10 + j);
            }
        }
        mi = min(mi, tmp);
    }
    println!("{}", mi);
}
