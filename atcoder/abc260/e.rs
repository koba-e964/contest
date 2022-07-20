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

// The author read the editorial before implementing this.
// Tags: intervals
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
        n: usize, m: usize,
        ab: [(usize1, usize1); n],
    }
    let mut app = vec![vec![]; m + 1];
    for i in 0..n {
        let (a, b) = ab[i];
        app[a].push(i);
        app[b].push(i);
    }
    let mut pop = vec![0; n];
    let mut zero = n;
    let mut ans = vec![0i64; m + 2];
    let mut r = 0;
    for i in 0..m + 1 {
        while r < m && zero > 0 {
            for &idx in &app[r] {
                pop[idx] += 1;
                if pop[idx] == 1 {
                    zero -= 1;
                }
            }
            r += 1;
        }
        if zero == 0 {
            ans[r - i] += 1;
            ans[m - i + 1] -= 1;
        }
        for &idx in &app[i] {
            pop[idx] -= 1;
            if pop[idx] == 0 {
                zero += 1;
            }
        }
    }
    for i in 1..m {
        ans[i + 1] += ans[i];
    }
    putvec!(ans[1..m + 1]);
}
