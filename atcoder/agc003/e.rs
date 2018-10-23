#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        qq: usize,
        q: [i64; qq],
    }
    let mut qs = vec![n as i64];
    for q in q {
        while let Some(x) = qs.last().cloned() {
            if x >= q { qs.pop(); }
            else { break; }
        }
        qs.push(q);
    }
    // eprintln!("qs = {:?}", qs);
    let mut que = BinaryHeap::new();
    let last = qs.pop().unwrap();
    let mut cur = 1i64;
    let mut curlen = last;
    for q in qs.into_iter().rev() {
        let oldcur = cur;
        cur *= (curlen / q) as i64;
        if curlen % q > 0 {
            que.push((curlen % q, oldcur));
        }
        curlen = q;
        while let Some((x, mul)) = que.pop() {
            if x >= q {
                cur += (x / q) * mul;
                if x % q > 0 {
                    que.push((x % q, mul));
                }
            } else {
                que.push((x, mul));
                break;
            }
        }
    }
    let mut ans = vec![0; n + 1];
    ans[0] += cur;
    ans[curlen as usize] -= cur;
    for (x, mul) in que {
        ans[0] += mul;
        ans[x as usize] -= mul;
    }
    for i in 0 .. n { ans[i + 1] += ans[i]; }
    for i in 0 .. n {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
