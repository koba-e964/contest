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
    input!(h: [i64; 3], w: [i64; 3]);
    if h[0] + h[1] + h[2] != w[0] + w[1] + w[2] {
        println!("0");
        return;
    }
    let mut ans = 0;
    for i in 1..30 {
        for j in 1..30 {
            for k in 1..30 {
                for l in 1..30 {
                    let mut x = [[i, j, h[0] - i - j], [k, l, h[1] - k - l], [0; 3]];
                    for y in 0..3 {
                        x[2][y] = w[y] - x[0][y] - x[1][y];
                    }
                    if (0..3).all(|i| (0..3).all(|j| x[i][j] > 0)) {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
