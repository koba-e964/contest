use std::cmp::*;
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
        h: i64, w: i64,
        n: usize,
        ab: [(i64, i64); n],
    }
    let mut hm = HashMap::new();
    for v in ab {
        *hm.entry(v).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (&(x, y), _) in &hm {
        for x in x - 1..x + 2 {
            if x <= 1 || x >= h { continue; }
            for y in y - 1..y + 2 {
                if y <= 1 || y >= w { continue; }
                let mut c = 0;
                for i in x - 1..x + 2 {
                    for j in y - 1..y + 2 {
                        c += *hm.get(&(i, j)).unwrap_or(&0);
                    }
                }
                ans = max(ans, c);
            }
        }
    }
    println!("{}", ans);
}
