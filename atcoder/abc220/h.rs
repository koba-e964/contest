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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn cliques(g: &[u64]) -> Vec<i64> {
    let n = g.len();
    let mut dp = vec![0; 1 << n];
    for bits in 0usize..1 << n {
        let mut s = 0;
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                s += (bits as u64 & g[i]).count_ones();
            }
        }
        if s % 4 == 0 {
            dp[bits] += 1;
        } else {
            dp[bits] -= 1;
        }
    }
    dp
}

fn main() {
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![0u64; n];
    for &(a, b) in &ab {
        g[a] |= 1 << b;
        g[b] |= 1 << a;
    }
    let mut fst = cliques(&g[..n / 2]);
    let mut ind = vec![0u64; n - n / 2];
    for i in n / 2..n {
        ind[i - n / 2] = g[i] >> (n / 2);
    }
    let snd = cliques(&ind);
    for i in 0..n / 2 {
        for bits in 0..1 << (n / 2) {
            if (bits & 1 << i) != 0 {
                continue;
            }
            let x = fst[bits];
            let y = fst[bits | 1 << i];
            fst[bits] = x + y;
            fst[bits | 1 << i] = x - y;
        }
    }
    let mut tot = 0;
    for bits in 0..1 << (n - n / 2) {
        let mut int = 0u64;
        for i in 0..n - n / 2 {
            if (bits & 1 << i) != 0 {
                int ^= g[n / 2 + i];
            }
        }
        int &= (1 << (n / 2)) - 1;
        tot += snd[bits] * fst[int as usize];
    }
    let mut s = 0;
    for i in 0..n {
        s += g[i].count_ones();
    }
    if s % 4 != 0 {
        tot = -tot;
    }
    println!("{}", ((1 << n) + tot) / 2);
}
