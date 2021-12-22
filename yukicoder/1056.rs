use std::cmp::*;
use std::io::{Write, BufWriter};
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

fn red(a: &[Vec<char>]) -> Vec<u64> {
    let n = a.len();
    let mut v = vec![vec![0; n]; n];
    match n {
        1 | 2 | 3 => return vec![0],
        _ => {}
    }
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == '#' {
                v[i][j] = 1;
            }
        }
    }
    for i in 0..2 {
        if v[i][0] == 1 {
            for j in 0..n {
                v[i][j] ^= 1;
            }
        }
    }
    for i in 1..2 {
        if v[0][i] == 1 {
            for j in 0..n {
                v[j][i] ^= 1;
            }
        }
    }
    if v[1][1] != 0 {
        for i in 0..3 {
            let j = 2 - i;
            v[i][j] ^= 1;
        }
    }
    for i in 2..n {
        if v[1][i] != 0 {
            for j in 0..n {
                v[j][i] ^= 1;
            }
        }
        if v[i][1] != 0 {
            for j in 0..n {
                v[i][j] ^= 1;
            }
        }
        if v[0][i] != 0 {
            for k in 0..n - i {
                v[k][i + k] ^= 1;
            }
        }
        if v[i][0] != 0 {
            for k in 0..n - i {
                v[i + k][k] ^= 1;
            }
        }
        if v[i][i] != 0 {
            for k in 0..min(n, 2 * i + 1) {
                if 2 * i - k < n {
                    v[2 * i - k][k] ^= 1;
                }
            }
        }
        if v[i - 1][i] != 0 {
            for k in 0..min(n, 2 * i) {
                if 2 * i - 1 - k < n {
                    v[2 * i - 1 - k][k] ^= 1;
                }
            }
        }
    }
    let mut ans = vec![0u64; 625];
    for i in 0..n {
        for j in 0..n {
            if v[i][j] != 0 {
                let idx = i * n + j;
                ans[idx / 64] |= 1 << (idx % 64);
            }
        }
    }
    ans
}

// https://yukicoder.me/problems/no/1056 (4.5)
// 何らかの標準形を見つけたい。​​
// N が小さい場合 (N <= 20) に操作で得られるものの空間の次元を求めると、1, 4, 6N-9 (N >= 3) であり、http://oeis.org/A270545 と同じだった。操作は全部で 6N - 2 種類あるので、この操作から自然に定義される線型写像 F_2^{6N-2} -> F_2^{N^2} の dim ker は 7 である。横、縦、斜め合計 4 通りで全体が作れるので、これらのうち 3 個余分なベクトルがある。
// N >= 3 でなんらかの標準形が作れたとして、N = N+1 の場合、新たに右下に追加される領域に対してだけ作用する (左上領域に制限したら 0 である) ベクトルがちょうど 6 個存在する: i = N + 1 の縦横, (1, N+1), (N+1, 1), {(N, N+1), (N+1, N)}, (N+1, N+1)。
// N >= 3 のときこれらは明らかに線型独立。よってこの 6 個のベクトルをつかって標準形を作れば良い。(2, N+1), (N+1, 2), (1, N+1), (N+1, 1), (N, N+1), (N+1, N+1) がこの順で 0 になるように操作していけば良い。
// 計算量は O(N^2 M + 等しさの計算量) で、等しさの計算量は愚直比較で O(N^2 M^2)、ハッシュなどを使って O(N^2 M + M^2) である。bitset にすれば愚直でも間に合うはず。(200^2 / 64 * 200^2 / 2 = 1.25 * 10^7 であるため。)
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [[chars; n]; m],
    }
    let mut b = vec![];
    for i in 0..m {
        b.push(red(&a[i]));
    }
    for i in 0..m - 1 {
        for j in i + 1..m {
            puts!("{}", if b[i] == b[j] { "1" } else { "0" });
        }
        puts!("\n");
    }
}
