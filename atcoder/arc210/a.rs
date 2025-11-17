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

fn main() {
    input! {
        n: usize, q: usize,
        ix: [(usize1, i64); q],
    }
    let mut gap = vec![0; n - 1];
    let mut delta = vec![0; n];
    for (i, x) in ix {
        delta[i] += x;
        if i > 0 {
            gap[i - 1] = gap[i - 1].max(delta[i - 1] - delta[i]);
        }
        if i + 1 < n {
            gap[i] = gap[i].max(delta[i] - delta[i + 1]);
        }
    }
    let mut a = vec![1; n];
    for i in 0..n - 1 {
        a[i + 1] = a[i] + gap[i] + 1;
    }
    let s: i64 = a.iter().sum();
    println!("{s}");
}
