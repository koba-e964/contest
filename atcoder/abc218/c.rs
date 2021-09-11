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
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn rot(a: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = a.len();
    let mut ret = vec![vec!['+'; n]; n];
    for i in 0..n {
        for j in 0..n {
            ret[n - j - 1][i] = a[i][j];
        }
    }
    ret
}

fn norm(a: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = a.len();
    let mut lx = n;
    let mut rx = 0;
    let mut ly = n;
    let mut ry = 0;
    for i in 0..n {
        for j in 0..n {
            if a[i][j] != '#' {
                continue;
            }
            lx.chmin(i);
            rx.chmax(i);
            ly.chmin(j);
            ry.chmax(j);
        }
    }
    let mut ret = vec![];
    for i in lx..rx + 1 {
        ret.push(a[i][ly..ry + 1].to_vec());
    }
    ret
}

fn main() {
    input! {
        n: usize,
        s: [chars; n],
        t: [chars; n],
    }
    let mut ok = false;
    let mut s = s;
    for _ in 0..4 {
        if norm(&s) == norm(&t) {
            ok = true;
        }
        s = rot(s);
    }
    println!("{}", if ok { "Yes" } else { "No" });
}
