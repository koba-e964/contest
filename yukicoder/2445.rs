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

#[inline]
fn madd(a: &mut i64, b: i64, m: i64) {
    let mut r = *a + b;
    if r >= m {
        r -= m;
    }
    *a = r
}

fn main() {
    input! {
        n: usize, b: i64,
        a: [[i64; n]; n],
    }
    let mut ac = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            ac[i][j] = a[i][j] % b;
        }
    }
    let mut dp = vec![[0; 2]; 1 << n];
    dp[0][0] += 1;
    for i in 0..n {
        let mut ep = vec![[0; 2]; 1 << n];
        for bits in 0usize..1 << n {
            if dp[bits] == [0; 2] { continue; }
            for j in 0..n {
                if (bits & 1 << j) != 0 { continue; }
                let flip = (bits & ((1 << n) - (1 << (j + 1)))).count_ones() % 2;
                let flip = flip as usize;
                for c in 0..2 {
                    let val = dp[bits][c] * ac[i][j] % b;
                    madd(&mut ep[bits | 1 << j][c ^ flip], val, b);
                }
            }
        }
        dp = ep;
    }
    println!("{}", dp[(1 << n) - 1][1]);
}
