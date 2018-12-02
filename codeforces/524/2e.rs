#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        m: usize,
        s: [chars; n],
    }
    let s: Vec<Vec<usize>> = s.into_iter().map(|x| {
        x.into_iter().map(|y| ((y as u8) - b'a') as usize).collect()
    }).collect();
    let mut rnd = vec![0u64; 26];
    {
        use std::hash::{Hasher, BuildHasher};
        let a = 0xdead_c0de_0013_3331;
        let b = 2457;
        let hm: HashMap<i32, i32> = HashMap::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        let mut x: u64 = hash.finish();
        for i in 0 .. 26 {
            x = x.wrapping_mul(a).wrapping_add(b);
            rnd[i] = x ^ x << 10;
        }
    }
    let mut tot = 0i64;
    for j in 0 .. m {
        let mut occ = vec![0u64; n];
        let mut odd = vec![0i32; n];
        let mut tmp = vec![0; 2 * n - 1];
        for k in j .. m {
            for i in 0 .. n {
                occ[i] = occ[i].wrapping_add(rnd[s[i][k]]);
                odd[i] ^= 1 << s[i][k];
                if (odd[i] & -odd[i]) == odd[i] {
                    tmp[2 * i] = occ[i];
                } else {
                    tmp[2 * i] = (i + 1) as u64; // Unique values
                }
            }
            // Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
            let mut r = vec![0; 2 * n - 1];
            {
                let mut i = 0;
                let mut j = 0;
                while i < 2 * n - 1 {
                    while i >= j && i + j < 2 * n - 1 && tmp[i - j] == tmp[i + j] {
                        j += 1;
                    }
                    r[i] = j;
                    let mut k = 1;
                    while i >= k && i + k < 2 * n - 1 && k + r[i - k] < j {
                        r[i + k] = r[i - k];
                        k += 1;
                    }
                    i += k;
                    j -= k;
                }
            }
            for i in 0 .. 2 * n - 1 {
                if i % 2 == 0 {
                    r[i] = (r[i] + 1) / 2;
                } else {
                    r[i] /= 2;
                }
            }
            // eprintln!("(j, k) = ({}, {}), r = {:?}", j, k, r);
            // eprintln!("tmp = {:?}", tmp);
            for i in 0 .. 2 * n - 1 {
                if i % 2 == 0 && tmp[i] == i as u64 / 2 + 1 { continue; } // dummy
                tot += r[i] as i64;
            }
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
