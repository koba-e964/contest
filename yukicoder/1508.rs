use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1508 (3)
// 逆から見る。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        a: [usize; q],
        b: [usize; q],
    }
    let mut v = vec![];
    let mut ch = vec![vec![]; q];
    for i in (0..q).rev() {
        v.push((a[i], a[i] + 1));
        v.push((b[i], b[i] + 1));
        v.sort();
        let mut nv = vec![];
        let mut st = 0;
        let mut last = 0;
        for &(l, r) in &v {
            if last < l {
                if last > 0 {
                    nv.push((st, last));
                }
                st = l;
                last = r;
            } else {
                last = max(last, r);
            }
        }
        nv.push((st, last));
        v.clear();
        for &(l, r) in &nv {
            if l == 1 && r == n + 1 {
                v.push((1, n + 1));
            } else if l == 1 {
                if r - 1 > 1 {
                    v.push((1, r - 1));
                }
            } else if r == n + 1 {
                if l < n {
                    v.push((l + 1, n + 1));
                }
            } else {
                if l + 1 < r - 1 {
                    v.push((l + 1, r - 1));
                }
            }
        }
        ch[i] = nv.clone();
    }
    if v == [(1, n + 1)] {
        puts!("NO\n");
        return;
    }
    let (l, r) = ch[0][0];
    let mut vac = if r == n + 1 {
        l - 1
    } else {
        r
    };
    puts!("YES\n{}\n", vac);
    for i in 0..q {
        let mut ok = 0;
        for j in max(1, vac - 1)..min(n, vac + 1) + 1 {
            if ch[i].iter().all(|&(l, r)| j < l || r <= j) {
                ok = j;
                break;
            }
        }
        vac = ok;
        puts!("{}\n", vac);
    }
}
