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

// Tags: moebius-transform, game, mimicking
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [usize; n],
    }
    const W: usize = 200_001;
    let mut pfac = vec![0; W];
    for p in 2..W {
        if pfac[p] != 0 {
            continue;
        }
        for i in 1..=(W - 1) / p {
            pfac[i * p] = p;
        }
    }
    let mut divs = vec![vec![]; W];
    for i in 2..W {
        for j in 1..=(W - 1) / i {
            divs[i * j].push(i);
        }
    }

    let mut cnt = vec![false; W];
    let mut pop = vec![0; W];
    for &a in &a {
        cnt[a] ^= true;
        pop[a] = 1;
    }
    for p in 2..W {
        if pfac[p] != p { continue; }
        for i in (1..=(W - 1) / p).rev() {
            cnt[i] ^= cnt[i * p];
            pop[i] += pop[i * p];
        }
    }
    let mut dp = vec![[false; 2]; W];
    for i in 2..W {
        let mut ps = vec![];
        {
            let mut x = i;
            while x > 1 {
                let p = pfac[x];
                ps.push(p);
                while x % p == 0 {
                    x /= p;
                }
            }
        }
        for b in if cnt[i] { [1, 0] } else { [0, 1] } {
            let mut win = false;
            for &d in &divs[i] {
                let mut occ = 0;
                for bits in 0..1 << ps.len() {
                    let mut prod = d;
                    let mut coef = 1;
                    for i in 0..ps.len() {
                        if (bits & 1 << i) != 0 {
                            prod *= ps[i];
                            coef *= -1;
                        }
                    }
                    if i % prod != 0 { continue; }
                    occ += coef * pop[prod];
                }
                assert!(occ >= 0);
                if occ == 0 { continue; }
                if d < i || (cnt[i] ^ (b != 0)) {
                    if !dp[d][1 - b] {
                        win = true;
                        break;
                    }
                }
            }
            dp[i][b] = win;
        }
    }
    for i in 1..=6 {
        eprintln!("cnt[{i}] = {}", cnt[i]);
        eprintln!("dp[{i}] = {:?}", dp[i]);
    }
    for a in a {
        puts!("{}\n", if dp[a][1] {
            "Aoki"
        } else {
            "Takahashi"
        });
    }
}
