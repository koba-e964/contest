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
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut f = vec![0; 7];
    for a in a {
        f[a] += 1;
    }
    let mut x = k;
    let tr = [
        (1, 5),
        (2, 6),
        (3, 6),
        (4, 6),
        (5, 6),
    ];
    for &(a, b) in &tr {
        while x > 0 && f[a] >= 1 {
            f[a] -= 1;
            f[b] += 1;
            x -= 1;
        }
    }
    let mut ans = 0;
    for i in 1..7 {
        ans += i * f[i];
    }
    println!("{}", ans);
}
