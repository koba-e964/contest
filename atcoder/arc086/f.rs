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
fn chmax<T: Ord>(a: &mut T, b: T) -> bool {
    if *a >= b { return false; }
    *a = b;
    true
}

fn dfs(a: Vec<i64>, k: i64, memo: &mut HashMap<Vec<i64>, i64>) {
    {
        let entry = memo.entry(a.clone()).or_insert(-1);
        if !chmax(entry, k) { return }
    }
    if k == 0 { return };
    let half = a.iter().map(|&x| x / 2).collect();
    dfs(half, k - 1, memo);
    if k > 1 && a[0] > 0 {
        let half = a.iter().map(|&x| (x - 1) / 2).collect();
        dfs(half, k - 2, memo);
    }
}

const MOD: i64 = 1_000_000_007;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut a = a;
    a.sort();

    let mut memo = HashMap::new();
    dfs(a, k, &mut memo);
    // compress
    let mut comp = HashMap::new();
    for (a, k) in memo {
        let mi = a[0];
        let mut diff = vec![0; n - 1];
        for i in 0 .. n - 1 { diff[i] = a[i + 1] - mi; }
        let range = (max(0, mi - k), mi);
        let entry = comp.entry(diff).or_insert(Vec::new());
        entry.push(range);
    }
    let mut tot = 0;
    for (_, ranges) in comp {
        let mut events = Vec::new();
        for (x, y) in ranges {
            events.push((x, 1));
            events.push((y + 1, -1));
        }
        events.sort();
        let mut cur = 0;
        let mut last = 0;
        for (t, delta) in events {
            if cur > 0 {
                tot += t - last;
                tot %= MOD;
            }
            last = t;
            cur += delta;
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
