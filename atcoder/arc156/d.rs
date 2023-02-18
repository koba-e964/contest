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
    input! {
        n: usize, k: i64,
        a: [usize; n],
    }
    let mut dp = vec![0i64; 1 << 10];
    let mut pop = vec![0i64; 1 << 10];
    pop[0] = 1;
    for i in 0..40 {
        let mut ep = vec![0; 1 << 10];
        let mut qop = vec![0; 1 << 10];
        if (k & 1 << i) == 0 {
            for b in 0..1 << 10 {
                let newidx = b >> 1;
                ep[newidx] ^= dp[b];
                if pop[b] == 1 {
                    let carry = b & 1;
                    qop[newidx] ^= pop[b];
                    ep[newidx] ^= (carry as i64) << i;
                }
            }
            dp = ep;
            pop = qop;
            continue;
        }
        for b in 0..1 << 10 {
            for &a in &a {
                let newidx = (b + a) >> 1;
                ep[newidx] ^= dp[b];
                if pop[b] == 1 {
                    qop[newidx] ^= pop[b];
                    let carry = (b + a) & 1;
                    ep[newidx] ^= (carry as i64) << i;
                }
            }
        }
        dp = ep;
        pop = qop;
    }
    let mut ans = 0;
    for i in 0..1 << 10 {
        ans ^= dp[i];
        if pop[i] == 1 {
            ans ^= (i as i64) << 40;
        }
    }
    println!("{}", ans);
}
