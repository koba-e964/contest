use std::cmp::*;
use std::collections::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, k: usize,
        a: [usize1; k],
    }
    let mut ans = vec![];
    let mut que = BinaryHeap::new();
    for i in 0..k {
        let lo = if i == 0 { 0 } else { a[i - 1] + 1 };
        let hi = if i == k - 1 { n } else { a[i] + 1 };
        if i < k - 1 {
            for v in (lo..hi - 1).rev() {
                que.push(Reverse(v));
            }
            ans.push(hi);
            if let Some(Reverse(k)) = que.pop() {
                ans.push(k + 1);
            }
        } else {
            for v in (lo..hi).rev() {
                que.push(Reverse(v));
            }
            let mut nums = vec![];
            while let Some(Reverse(k)) = que.pop() {
                nums.push(k + 1);
            }
            nums.reverse();
            ans.extend(&nums);
        }
    }
    putvec!(ans);
}
