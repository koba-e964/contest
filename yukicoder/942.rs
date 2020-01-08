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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

// The author read the editorial before implementing this.
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, k: usize,
    }
    let q = n / k;
    if n % 2 == 0 {
        if q % 2 != 0 {
            puts!("No\n");
            return;
        }
        puts!("Yes\n");
        for i in 0..k {
            let mut ans = vec![];
            for j in 0..q / 2 {
                ans.push(1 + j + i * q / 2);
                ans.push(n - (j + i * q / 2));
            }
            for j in 0..q {
                puts!("{}{}", ans[j], if j + 1 == q { "\n" } else { " " });
            }
        }
        return;
    }
    assert_eq!(q % 2, 1);
    if q == 1 {
        if n == 1 {
            puts!("Yes\n1\n");
        } else {
            puts!("No\n");
        }
        return;
    }
    let mut ans = vec![vec![]; k];
    // 天才すぎる 無理 (無理か?)
    let three_sum = 3 * (3 * k + 1) / 2;
    for i in 0..k {
        let a0 = 1 + i;
        let a1 = k + 1 + ((k - 1) / 2 + i) % k;
        let a2 = three_sum - a0 - a1;
        ans[i].extend_from_slice(&[a0, a1, a2]);
    }
    for i in 0..(q - 3) / 2 {
        for j in 0..k {
            let a0 = 3 * k + 1 + i * k + j;
            let a1 = q * k - i * k - j;
            ans[j].extend_from_slice(&[a0, a1]);
        }
    }
    puts!("Yes\n");
    for i in 0..k {
        for j in 0..ans[i].len() {
            puts!("{}{}", ans[i][j],
                  if j + 1 == ans[i].len() { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
