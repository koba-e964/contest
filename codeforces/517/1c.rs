#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
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

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn naive(a: &[i32]) -> Option<Vec<(usize, usize)>> {
    const INF: i32 = 1 << 25;
    let n = a.len();
    let mut init = 0;
    for i in 0..n {
        init |= (a[i] as usize) << i;
    }
    let mut dist = vec![INF; 1 << n];
    let mut pre = vec![(0, 0, 0); 1 << n];
    let mut que = VecDeque::new();
    que.push_back((0, 0, 0, 0, 0));
    while let Some((d, v, k, s, pat)) = que.pop_front() {
        if dist[v] <= d { continue; }
        dist[v] = d;
        pre[v] = (k, s, pat);
        for i in 0..n {
            for s in 1..(n - i - 1) / 2 + 1 {
                let pat = 1 << i | 1 << (i + s) | 1 << (i + 2 * s);
                que.push_back((d + 1, v ^ pat, i, s, pat));
            }
        }
    }
    if dist[init] >= INF {
        return None;
    }
    let mut cur = init;
    let mut ops = vec![];
    while dist[cur] > 0 {
        let (k, s, pat) = pre[cur];
        ops.push((k, s));
        cur ^= pat;
    }
    Some(ops)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [i32; n],
    }
    let mut a = a;
    let mut op = vec![];
    let mut pos = 0;
    while pos + 9 < n {
        if a[pos] == 0 {
            pos += 1;
            continue;
        }
        if pos + 12 > n { break; }
        if a[pos + 1] == 0 {
            let idx = if a[pos + 2] == 0 {
                3
            } else {
                2
            };
            op.push((pos, idx));
            a[pos] ^= 1;
            a[pos + idx] ^= 1;
            a[pos + 2 * idx] ^= 1;
            pos += 3;
            continue;
        }
        if a[pos + 2] == 1 {
            op.push((pos, 1));
            a[pos] ^= 1;
            a[pos + 1] ^= 1;
            a[pos + 2] ^= 1;
            pos += 3;
            continue;
        }
        if pos + 15 > n { break; }
        let cur = match 4 * a[pos + 3] + 2 * a[pos + 4] + a[pos + 5] {
            // 110000...111111
            0 => [(pos, 4), (pos + 1, 3)],
            1 => [(pos, 3), (pos + 1, 2)],
            2 => [(pos, 4), (pos + 1, 5)],
            3 => [(pos, 4), (pos + 1, 4)],
            4 => [(pos, 5), (pos + 1, 2)],
            5 => [(pos, 3), (pos + 1, 4)],
            6 => [(pos, 3), (pos + 1, 3)],
            7 => [(pos, 4), (pos + 1, 2)],
            _ => panic!(),
        };
        for i in 0..2 {
            let (k, s) = cur[i];
            op.push(cur[i]);
            a[k] ^= 1;
            a[k + s] ^= 1;
            a[k + 2 * s] ^= 1;
        }
        for j in 0..6 {
            assert_eq!(a[pos + j], 0);
        }
        pos += 6;
    }
    let rest = naive(&a[pos..]);
    if rest == None {
        puts!("NO\n");
        return;
    }
    let rest = rest.unwrap();
    for &(t, s) in &rest {
        op.push((t + pos, s));
    }
    puts!("YES\n");
    puts!("{}\n", op.len());
    for &(k, s) in &op {
        puts!("{} {} {}\n", 1 + k, 1 + k + s, 1 + k + 2 * s);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
