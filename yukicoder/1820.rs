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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1820 (3.5)
// [-1, -2, ..., -2^{M-1}] を作るのに M 回
// [~1, ~2, ..., ~2^{M-1}] を作るのに 2M 回
// 1 bit 調整するのに 2 回
// 合計 5M 回程度の操作で可能。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        x: chars,
        _a: [chars; n],
    }
    let lim = 1_000_000_000;
    let mut k = 3 * m;
    for i in 0..m {
        if x[m - 1 - i] == '0' { k += 2; }
    }
    puts!("{}\n", k);
    puts!("2 {} {} {}\n", lim, lim, lim);
    for i in 1..m {
        puts!("1 {} {}\n", lim - i, lim - i + 1);
    }
    for i in 0..m - 1 { 
        puts!("2 {} {} {}\n", lim - i - m, lim - i - 1, lim - i - 1);
        puts!("2 {} {} {}\n", lim - i - m, lim - i, lim - i - m);
    }
    puts!("2 {} {} {}\n", lim - 2 * m + 1, lim - m + 1, lim - m + 1);
    puts!("2 0 0 {}\n", lim - 2 * m);
    for i in 0..m {
        if x[m - 1 - i] == '0' {
            puts!("2 0 0 {}\n", lim - m - i);
            puts!("2 0 0 0\n");
        }
    }
}
