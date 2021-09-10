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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: mos-algorithm
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize, p: i64,
        a: [usize; n],
        st: [(usize1, usize); q],
    }
    const W: usize = 1_000_010;
    const B: usize = 320;
    let mut l = 0;
    let mut r = 0;
    let mut val = vec![0; W];
    let mut memo = vec![vec![0]; W];
    let mut pos = vec![0; W];
    let mut sum = 0;
    let mut sti = vec![];
    for i in 0..q {
        sti.push((st[i], i));
    }
    sti.sort_by_key(|&((s, t), _)| {
        let q = s / B;
        if q % 2 == 0 {
            (q, t)
        } else {
            (q, (1 << 30) - t)
        }
    });
    let mut ans = vec![-1; q];
    for ((s, t), idx) in sti {
        while r < t {
            let x = a[r];
            let oldval = val[x];
            if pos[x] + 1 == memo[x].len() {
                memo[x].push((oldval + 1) * x as i64 % p);
            }
            pos[x] += 1;
            val[x] = memo[x][pos[x]];
            sum = (sum + val[x] - oldval + p) % p;
            r += 1;
        }
        while l > s {
            l -= 1;
            let x = a[l];
            let oldval = val[x];
            if pos[x] + 1 == memo[x].len() {
                memo[x].push((oldval + 1) * x as i64 % p);
            }
            pos[x] += 1;
            val[x] = memo[x][pos[x]];
            sum = (sum + val[x] - oldval + p) % p;
        }
        while r > t {
            r -= 1;
            let x = a[r];
            let oldval = val[x];
            pos[x] -= 1;
            val[x] = memo[x][pos[x]];
            sum = (sum + val[x] - oldval + p) % p;
        }
        while l < s {
            let x = a[l];
            let oldval = val[x];
            pos[x] -= 1;
            val[x] = memo[x][pos[x]];
            sum = (sum + val[x] - oldval + p) % p;
            l += 1;
        }
        ans[idx] = sum;
    }
    for i in 0..q {
        puts!("{}\n", ans[i]);
    }
}
