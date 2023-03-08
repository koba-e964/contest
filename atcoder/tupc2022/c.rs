use std::collections::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        _h: usize, _w: usize, n: usize,
        xy: [(usize, usize); n],
    }
    let mut hm = HashMap::new();
    for (x, y) in xy {
        *hm.entry((x, y)).or_insert(0) ^= 1;
        if x > 1 {
            *hm.entry((x - 1, y)).or_insert(0) ^= 1;
        }
        if y > 1 {
            *hm.entry((x, y - 1)).or_insert(0) ^= 1;
        }
        if x > 1 && y > 1 {
            *hm.entry((x - 1, y - 1)).or_insert(0) ^= 1;
        }
    }
    let s: i32 = hm.values().sum();
    println!("{}", s);
}
