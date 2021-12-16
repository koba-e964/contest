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
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/1384 (3)
// N, M が共に偶数であれば、(1, 2), (2, 1), (1, 1), (2, 2), (2, 3) という動き方で 2 \times 2 のマスを埋めながら隣の 2 \times 2 のマスに移動できるので、可能。
// (N, M) != (1, 1) かつ N, M のどちらが奇数の時できないことを示す。(x, y) について、(x mod 2, y mod 2) で分類することを考える。a = floor(N / 2), b = floor(M / 2) とおく。1 の動きができるのは min(ab, (N-a)(M-b)) + min(a(M-b), (N-a)b) 回までなので、この値を x とおくと 2x - 1 <= Q <= 2x, Q = NM - 1 が成立することから、2x + 1 < NM を示せば不可能性が証明できる。
// 一般性を失わず 2a + 1 = N であると仮定してよい。x = min(ab, (a+1)(M-b)) + min(a(M-b), (a+1)b) <= ab + a(M-b) <= aM より、2x + 1 <= 2aM + 1 <= (2a + 1)M = NM である。中央の不等号が等号になるのは M = 1 のときに限られ、その場合は b = 0 であるため x = 0 + 0 = 0 であり、2x + 1 = 1 であるため、NM != 1 でない限り 2x + 1 < NM が成立する。
// よって、(N, M) = (1, 1) または N と M が両方偶数の場合、およびその場合に限り可能。
// Tags: constant-size-input
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        t: usize,
        nm: [(usize, usize); t],
    }
    for (n, m) in nm {
        if (n, m) == (1, 1) {
            puts!("0\n1 1\n");
            continue;
        }
        if n % 2 != 0 || m % 2 != 0 {
            puts!("-1\n");
            continue;
        }
        let mut ans = vec![];
        for i in 0..n / 2 {
            if i % 2 == 0 {
                for j in 0..m / 2 - 1 {
                    ans.push((2 * i, 2 * j));
                    ans.push((2 * i + 1, 2 * j + 1));
                    ans.push((2 * i + 1, 2 * j));
                    ans.push((2 * i, 2 * j + 1));
                }
                let j = m / 2 - 1;
                ans.push((2 * i, 2 * j));
                ans.push((2 * i + 1, 2 * j + 1));
                ans.push((2 * i, 2 * j + 1));
                ans.push((2 * i + 1, 2 * j));
            } else {
                let j = m / 2 - 1;
                ans.push((2 * i, 2 * j));
                ans.push((2 * i + 1, 2 * j + 1));
                ans.push((2 * i, 2 * j + 1));
                ans.push((2 * i + 1, 2 * j));
                for j in (0..m / 2 - 1).rev() {
                    ans.push((2 * i + 1, 2 * j + 1));
                    ans.push((2 * i, 2 * j));
                    ans.push((2 * i, 2 * j + 1));
                    ans.push((2 * i + 1, 2 * j));
                }
            }
        }
        assert_eq!(ans.len(), n * m);
        puts!("{}\n", ans.len() - 1);
        for (a, b) in ans {
            puts!("{} {}\n", a + 1, b + 1);
        }
    }
}
