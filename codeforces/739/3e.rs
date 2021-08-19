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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn calc(t: &[char]) -> Option<(String, String)> {
    let mut ord = vec![];
    let mut f = vec![0; 26];
    for &c in t.iter().rev() {
        let idx = (c as u8 - b'a') as usize;
        if f[idx] == 0 {
            ord.push(idx);
        }
        f[idx] += 1;
    }
    ord.reverse();
    let k = ord.len();
    let mut sum = 0;
    for i in 0..k {
        let coef = i + 1;
        if f[ord[i]] % coef != 0 {
            return None;
        }
        sum += f[ord[i]] / coef;
    }
    let s = t[..sum].to_vec();
    let mut cur = s.clone();
    let mut u: Vec<char> = vec![];
    for i in 0..k {
        u.extend(&cur);
        let c = (b'a' + ord[i] as u8) as char;
        cur = cur.into_iter().filter(|&d| d != c).collect();
    }
    if u == t {
        Some((s.into_iter().collect(), ord.iter().map(|&x| (b'a' + x as u8) as char).collect()))
    } else {
        None
    }
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input!(t: [chars]);
    for t in t {
        if let Some((u, v)) = calc(&t) {
            puts!("{} {}\n", u, v);
        } else {
            puts!("-1\n");
        }
    }
}
