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

fn dfs(a: &[Vec<i64>], b: [[i8; 3]; 3]) -> bool {
    // 3 in a row?
    for i in 0..3 {
        if b[i].iter().all(|&b| b == -1) {
            return false;
        }
        if (0..3).all(|j| b[j][i] == -1) {
            return false;
        }
    }
    if (0..3).all(|j| b[j][j] == -1) {
        return false;
    }
    if (0..3).all(|j| b[j][2 - j] == -1) {
        return false;
    }
    if b.iter().all(|b| b.iter().all(|&b| b != 0)) {
        // check
        let mut sum = 0;
        for i in 0..3 {
            for j in 0..3 {
                sum += a[i][j] * b[i][j] as i64;
            }
        }
        return sum > 0;
    }
    let mut win = false;
    for i in 0..3 {
        for j in 0..3 {
            if b[i][j] != 0 { continue; }
            let mut newb = b;
            for x in 0..9 {
                newb[x / 3][x % 3] *= -1;
            }
            newb[i][j] = -1;
            if !dfs(a, newb) {
                win = true;
            }
        }
    }
    win
}

fn main() {
    input!(a: [[i64; 3]; 3]);
    let b = [[0; 3]; 3];
    let res = dfs(&a, b);
    println!("{}", if res { "Takahashi" } else { "Aoki" });
}
