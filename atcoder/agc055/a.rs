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
        n: usize,
        s: chars,
    }
    let mut acc = vec![[0; 3]; 3 * n + 1];
    for i in 0..3 * n {
        let idx = (s[i] as u8 - b'A') as usize;
        acc[i + 1] = acc[i];
        acc[i + 1][idx] += 1;
    }
    let mut a = -1;
    for i in 0..n as i64 + 1 {
        let b = acc[n][0] - i;
        let c = acc[3 * n][2] - acc[2 * n][2] - i;
        let d = acc[n][1] - c;
        let e = acc[2 * n][0] - acc[n][0] - c;
        let f = acc[n][2] - e;
        if b >= 0 && c >= 0 && d >= 0 && e >= 0 && f >= 0
            && b + e == acc[3 * n][1] - acc[2 * n][1]
            && d + f == acc[3 * n][0] - acc[2 * n][0]
            && i + f == acc[2 * n][1] - acc[n][1]
            && b + d == acc[2 * n][2] - acc[n][2] {
                a = i;
                break;
            }
    }
    let b = acc[n][0] - a;
    let c = acc[3 * n][2] - acc[2 * n][2] - a;
    let d = acc[n][1] - c;
    let e = acc[2 * n][0] - acc[n][0] - c;
    let f = acc[n][2] - e;
    eprintln!("{} {} {} {} {} {}", a, b, c, d, e, f);
    let mut occ = vec![vec![vec![]; 3]; 3];
    for i in 0..3 * n {
        let idx = (s[i] as u8 - b'A') as usize;
        occ[i / n][idx].push(i);
    }
    let mut ans = vec!['0'; 3 * n];
    let ord = vec![[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let len = [a, b, c, d, e, f];
    let mut pos = vec![[0; 3]; 3];
    for i in 0..6 {
        for j in 0..3 {
            for _ in 0..len[i] {
                let u = ord[i][j];
                ans[occ[j][u][pos[j][u]]] = (b'1' + i as u8) as char;
                pos[j][u] += 1;
            }
        }
    }
    puts!("{}\n", ans.into_iter().collect::<String>());
}
