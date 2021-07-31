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
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// u64: MSB ...000111... LSB is considered to be sorted.
fn dfs(n: usize, idx: &[u64], one: &[u64], indet: u64) -> bool {
    eprintln!("idx: {}, |one| = {}", idx.len(), one.len());
    if idx.is_empty() {
        return indet == 0 && one.iter().all(|&one| ((one + 1) & one) == 0);
    }
    let mask = idx[0];
    let mut p = vec![];
    for i in 0..n {
        if (mask & 1 << i) != 0 {
            p.push(i);
        }
    }
    let mut next_one = vec![];
    for &one in one {
        assert_eq!(one & indet, 0);
        let ones = (one & mask).count_ones();
        let indets = (indet & mask).count_ones();
        let mut tmp = 0u64;
        let mut pos = 0;
        for i in ones..ones + indets + 1 {
            while pos < i as usize {
                tmp |= 1 << p[pos];
                pos += 1;
            }
            next_one.push((one & !mask) | tmp);
        }
    }
    if idx.len() >= 2 {
        next_one.sort_unstable(); next_one.dedup();
    }
    dfs(n, &idx[1..], &next_one, indet & !mask)
}

// The author implemented this after reading the solution.
// Tags: tight-time-limit, constant-factor-optimization, unusual-complexity
fn main() {
    input! {
        n: usize, k: usize,
        idx: [[usize1]; k],
    }
    if n == 1 {
        println!("ACCEPTED");
        return;
    }
    let idx: Vec<_> = idx.iter().map(|x| {
        let mut c = 0u64;
        for &v in x {
            c |= 1 << (n - 1 - v);
        }
        c
    }).collect();
    let ok = dfs(n, &idx, &[0], (1 << n) - 1);
    println!("{}", if ok { "ACCEPTED" } else { "REJECTED" });
}
