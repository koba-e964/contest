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

// e.g., k = 3, 7 -> 1 6 -> 6 -> 3 3 -> 3 -> {}
fn dfs(a: i64, k: i64) -> i64 {
    if a <= k {
        return a;
    }
    if a % 2 == 0 {
        0
    } else {
        -1
    }
}

fn main() {
    input! {
        n: usize, k: i64,
        a: [i64; n],
    }
    let sum: i64 = a.iter().sum();
    let mut prof = vec![];
    for a in a {
        prof.push(dfs(a, k));
    }
    prof.sort(); prof.reverse();
    let mut ans = 0;
    for i in 0..n {
        if i % 2 == 0 {
            ans += prof[i];
        } else {
            ans -= prof[i];
        }
    }
    println!("{}", (sum + ans) / 2);
}
