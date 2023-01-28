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
        s: chars,
        t: chars,
    }
    let mt = |a: char, b: char| {
        a == b || a == '?' || b == '?'
    };
    let m = t.len();
    let mut pre = vec![false; m + 1];
    pre[0] = true;
    for i in 0..m {
        if mt(s[i], t[i]) {
            pre[i + 1] = pre[i];
        }
    }
    let mut suf = vec![false; m + 1];
    suf[0] = true;
    for i in 0..m {
        if mt(s[s.len() - 1 - i], t[m - 1 - i]) {
            suf[i + 1] = suf[i];
        }
    }
    for i in 0..m + 1 {
        puts!("{}\n", if pre[i] && suf[m - i] { "Yes" } else { "No" });
    }
}
