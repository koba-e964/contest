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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1901 (4)
// アダマール変換 -> 点ごとの積 -> アダマール変換 -> 2^n で割る をすればよく、
// そのために各点で n+1 整数係数の 32 次未満・64 次未満の多項式を持ちたい。
// これは 32bit 整数・64bit 整数を n+1 個持つ方針でできる。
// 計算量は O(2^n n^2)、ただし _mm_clmulepi64_si128 を 2^n (n^2/2 + O(n)) 回呼ぶ。
// -> 破綻している。この場合 F_2[x] の乗算ではなく (Z/2^{n+1})[x] の乗算を行う必要があった。
// 単に 32^2 回演算して AC できた。
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        a: [[i32; 32]; 1 << n],
        b: [[i32; 32]; 1 << n],
    }
    let mut x = vec![[0u32; 32]; 1 << n];
    let mut y = vec![[0u32; 32]; 1 << n];
    for i in 0..1 << n {
        for j in 0..32 {
            if a[i][j] == 1 {
                x[i][j] = 1;
            }
        }
        for j in 0..32 {
            if b[i][j] == 1 {
                y[i][j] = 1;
            }
        }
    }
    for i in 0..n {
        for bits in 0..1 << n {
            if (bits & 1 << i) != 0 { continue; }
            for u in 0..32 {
                let p = x[bits][u];
                let q = x[bits | 1 << i][u];
                x[bits][u] = p.wrapping_add(q);
                x[bits | 1 << i][u] = p.wrapping_sub(q);
                let p = y[bits][u];
                let q = y[bits | 1 << i][u];
                y[bits][u] = p.wrapping_add(q);
                y[bits | 1 << i][u] = p.wrapping_sub(q);
            }
        }
    }
    let mut prod = vec![[0u32; 63]; 1 << n];
    for bits in 0..1 << n {
        for i in 0..32 {
            for j in 0..32 {
                prod[bits][i + j] = prod[bits][i + j].wrapping_add(x[bits][i].wrapping_mul(y[bits][j]));
            }
        }
    }
    for i in 0..n {
        for bits in 0..1 << n {
            if (bits & 1 << i) != 0 { continue; }
            for u in 0..63 {
                let p = prod[bits][u];
                let q = prod[bits | 1 << i][u];
                prod[bits][u] = p.wrapping_add(q);
                prod[bits | 1 << i][u] = p.wrapping_sub(q);
            }
        }
    }
    for v in &mut prod {
        for j in 0..63 {
            v[j] = (v[j] >> n) & 1;
        }
        putvec!(v);
    }
}
