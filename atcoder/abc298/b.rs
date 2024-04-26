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

fn rot(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let mut b = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = a[n - 1 - j][i];
        }
    }
    b
}

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
        b: [[i32; n]; n],
    }
    let mut a = a;
    for _ in 0..4 {
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 && b[i][j] == 0 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        a = rot(a);
    }
    println!("No");
}
