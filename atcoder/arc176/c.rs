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

fn main() {
    input! {
        n: usize, m: usize,
        abc: [(usize1, usize1, usize1); m],
    }
    let mut ev = vec![vec![]; n];
    for (a, b, c) in abc {
        ev[c].push((a, b));
    }
    const MOD: i64 = 998_244_353;
    let mut ans = 1;
    let mut last = 0;
    let mut dec = vec![false; n];
    for i in 0..n {
        if ev[i].is_empty() {
            continue;
        }
        let get = |j: usize| {
            let mut v = vec![];
            let (k1, k2) = ev[i][j];
            if !dec[k1] {
                v.push(k1);
            }
            if !dec[k2] {
                v.push(k2);
            }
            v
        };
        let mut int = get(0);
        let mut uni = get(0);
        for j in 1..ev[i].len() {
            let mut new_int = vec![];
            let cur = get(j);
            for &x in &int {
                if cur.contains(&x) {
                    new_int.push(x);
                }
            }
            int = new_int;
            uni.extend_from_slice(&cur);
        }
        if int.is_empty() {
            println!("0");
            return;
        }
        if int.len() == 2 {
            assert_eq!(uni.len(), 2);
            for &v in &uni {
                assert!(!dec[v]);
                dec[v] = true;
            }
            if last >= i {
                ans = 0;
                continue;
            }
            ans = ans * (i - last) as i64 % MOD * 2 % MOD;
            last += 2;
            continue;
        }
        assert_eq!(int.len(), 1);
        uni.sort(); uni.dedup();
        for &v in &uni {
            assert!(!dec[v]);
            dec[v] = true;
        }
        // *= P(i - last, uni.len() - 1)
        if last + uni.len() - 1 > i {
            ans = 0;
            continue;
        }
        for j in 1..uni.len() {
            ans = ans * (i + 1 - last - j) as i64 % MOD;
        }
        last += uni.len();
    }
    for i in last..n {
        ans = ans * (n - i) as i64 % MOD;
    }
    println!("{}", ans);
}
