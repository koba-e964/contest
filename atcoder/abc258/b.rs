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
        a: [chars; n],
    }
    let mut ma = "".to_string();
    for i in 0..n {
        for j in 0..n {
            for dx in -1..2 {
                for dy in -1..2 {
                    if (dx, dy) == (0, 0) { continue; }
                    let mut x = i;
                    let mut y = j;
                    let mut v = "".to_string();
                    for _ in 0..n {
                        v.push(a[x][y]);
                        x = (x as i32 + dx + n as i32) as usize % n;
                        y = (y as i32 + dy + n as i32) as usize % n;
                    }
                    ma = max(ma, v);
                }
            }
        }
    }
    println!("{}", ma);
}
