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

fn rec(a: &[u32], pos: usize) -> u32 {
    if pos == 0 {
        return 0;
    }
    assert!(!a.is_empty());
    let n = a.len();
    if (a[0] >> (pos - 1) & 1) == (a[n - 1] >> (pos - 1) & 1) {
        return rec(a, pos - 1);
    }
    let mut x = 0;
    while (a[x] >> (pos - 1) & 1) == 0 {
        x += 1;
    }
    min(rec(&a[..x], pos - 1), rec(&a[x..], pos - 1)) | 1 << (pos - 1)
}

// Tags: k-smallest-elements
fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let mut a = a;
    a.sort();
    println!("{}", rec(&a, 30));
}
