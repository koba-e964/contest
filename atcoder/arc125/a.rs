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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    input! {
        n: usize, m: usize,
        a: [i32; n],
        b: [i32; m],
    }
    let mut a = a;
    let mut b = b;
    if a[0] == 1 {
        for i in 0..n {
            a[i] ^= 1;
        }
        for i in 0..m {
            b[i] ^= 1;
        }
    }
    if a.iter().all(|&x| x == 0) {
        if b.iter().any(|&x| x != 0) {
            println!("-1");
            return;
        }
        println!("{}", m);
        return;
    }
    if b.iter().all(|&x| x == 0) {
        println!("{}", m);
        return;
    }
    let mut t = n;
    for i in 0..n {
        if a[i] == 1 {
            t.chmin(i);
            t.chmin(n - i);
        }
    }
    let mut tot = m;
    if b[0] == 0 {
        tot += t - 1;
    } else {
        tot += t;
    }
    for i in 0..m - 1 {
        if b[i] != b[i + 1] {
            tot += 1;
        }
    }
    println!("{}", tot);
}
