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

fn main() {
    input! {
        n: i64, m: usize,
        x: [i64; m],
        a: [i64; m],
    }
    let asum = a.iter().sum::<i64>();
    if asum != n {
        println!("-1");
        return;
    }
    let mut xa = vec![];
    for i in 0..m {
        xa.push((x[i], a[i]));
    }
    xa.sort();
    let mut tot = 0;
    for i in (0..m).rev() {
        let (x, a) = xa[i];
        if tot + a + x > n + 1 {
            println!("-1");
            return;
        }
        tot += a;
    }
    let mut ans = n * (n + 1) / 2;
    for (x, a) in xa {
        ans -= x * a;
    }
    println!("{ans}");
}
