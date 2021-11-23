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

/// Generates an Iterator over subsets of univ, in the descending order. 
/// Verified by: http://judge.u-aizu.ac.jp/onlinejudge/review.jsp?rid=3050308
struct SubsetIter { bits: Option<usize>, univ: usize }
impl Iterator for SubsetIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self.bits {
            None => None,
            Some(bits) => {
                let ans = bits;
                self.bits =
                    if bits == 0 { None }
                else { Some((bits - 1) & self.univ) };
                Some(ans)
            }
        }
    }
}
fn subsets(univ: usize) -> SubsetIter {
    SubsetIter { bits: Some(univ), univ: univ }
}

fn find(c: usize, one: usize, two: usize, three: usize) -> usize {
    let mut vis = (1 << c) - one - two - three - 1;
    loop {
        let nv = vis | ((vis << 1 | vis >> 1) & one);
        if nv == vis { break; }
        vis = nv;
    }
    let mut added = 0;
    for i in 0..c {
        if (two & 1 << i) != 0 && ((vis << 1) & (5 << i)) == 5 << i {
            added |= 1 << i;
        }
    }
    vis | added
}

// https://yukicoder.me/problems/no/309 (4)
// bitDP で O(R4^C C)。-> TLE。前計算をし、高速化をすることで O(4^C (R + C) + 3^C R) にして AC。-> 前計算を無駄に何回もやっていた。無駄を取り除いて 0.5 sec。
// Tags: constant-factor-optimization
fn main() {
    input! {
        r: usize, c: usize,
        p: [[f64; c]; r],
        s: [[i32; c]; r],
    }
    let mut memo = vec![vec![0; 1 << c]; 1 << c];
    for i in 0..1 << c {
        for j in 0..1 << c {
            memo[i][j] = find(c, i & !j, !i & j, i & j);
        }
    }
    let mut dp = vec![0.0; 1 << c];
    let mut dpx = vec![0.0; 1 << c];
    dp[0] = 1.0;
    for i in 0..r {
        let mut ep = vec![0.0; 1 << c];
        let mut epx = vec![0.0; 1 << c];
        for bits in 0..1 << c {
            let mut rem = vec![4; c];
            let mut pr = 1.0;
            for j in 0..c {
                if (bits & 1 << j) != 0 {
                    rem[j] = s[i][j];
                    pr *= p[i][j] / 100.0;
                } else {
                    pr *= 1.0 - p[i][j] / 100.0;
                }
            }
            let mut sum = vec![0.0; 1 << c];
            let mut sumx = vec![0.0; 1 << c];
            for pre in 0..1 << c {
                let to = pre & bits;
                sum[to] += dp[pre];
                sumx[to] += dpx[pre];
            }
            for pre in subsets(bits) {
                let sum = sum[pre];
                let sumx = sumx[pre];
                if sum == 0.0 { continue; }
                for j in 0..c {
                    if (pre & 1 << j) != 0 {
                        rem[j] -= 1;
                    }
                }
                let mut one = 0;
                let mut two = 0;
                let mut three = 0;
                for i in 0..c {
                    if rem[i] == 1 {
                        one |= 1 << i;
                    }
                    if rem[i] == 2 {
                        two |= 1 << i;
                    }
                    if rem[i] >= 3 {
                        three |= 1 << i;
                    }
                }
                let to = memo[one | three][two | three];
                ep[to] += sum * pr;
                epx[to] += sumx * pr + sum * pr * to.count_ones() as f64;
                for j in 0..c {
                    if (pre & 1 << j) != 0 {
                        rem[j] += 1;
                    }
                }
            }
        }
        dp = ep;
        dpx = epx;
    }
    let s: f64 = dpx.iter().sum();
    println!("{}", s);
}
