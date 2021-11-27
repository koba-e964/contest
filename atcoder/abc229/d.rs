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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        s: chars,
        k: usize,
    }
    let n = s.len();
    let mut ans = 0;
    let mut pos = 0;
    let mut cur = 0;
    for i in 0..n {
        while pos < n && cur + if s[pos] == 'X' { 0 } else { 1 } <= k {
            cur += if s[pos] == 'X' { 0 } else { 1 };
            pos += 1;
        }
        ans = max(ans, pos - i);
        if s[i] == '.' {
            cur -= 1;
        }
    }
    println!("{}", ans);
}
