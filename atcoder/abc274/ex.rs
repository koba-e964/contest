fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "Yes"
    } else {
        "No"
    }
}

struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm = std::collections::HashMap::<i32, i32>::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

mod gf {
    const POLY: u64 = 0x1b;

    const fn xtime(x: u64) -> u64 {
        x.wrapping_shl(1) ^ (x >> 63).wrapping_neg() & POLY
    }

    const fn make_red() -> [u64; 16] {
        let mut red = [0; 16];
        let mut i = 1;
        while i < 16 {
            let mut x = (i as u64) << 60;
            let mut j = 0;
            while j < 4 {
                x = xtime(x);
                j += 1;
            }
            red[i] = x;
            i += 1;
        }
        red
    }

    const RED: [u64; 16] = make_red();

    #[allow(dead_code)]
    pub fn mul_slow(mut a: u64, mut b: u64) -> u64 {
        let mut x: u64 = 0;
        for _ in 0..64 {
            x ^= a & (b & 1).wrapping_neg();
            b >>= 1;
            a = xtime(a);
        }
        x
    }

    pub fn mul_fast(a: u64, mut b: u64) -> u64 {
        let mut table = [0; 16];
        table[1] = a;
        for i in 2..16 {
            table[i] = if i & 1 == 0 {
                xtime(table[i >> 1])
            } else {
                table[i - 1] ^ a
            };
        }

        let mut x: u64 = 0;
        for _ in 0..16 {
            let digit = (b >> 60) as usize;
            b <<= 4;
            x = x.wrapping_shl(4) ^ RED[(x >> 60) as usize] ^ table[digit];
        }
        x
    }
}

use gf::mul_fast as gfmul;

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let arr = getline().trim().split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut rng = Rng::new();
    let base = rng.next() as u64;
    let base = base << 32 | rng.next() as u64;

    let mut pw = vec![1; n + 1];
    for i in 0..n {
        pw[i + 1] = gfmul(pw[i], base);
    }
    let mut hash = vec![0; n + 1];
    for i in 0..n {
        hash[i + 1] = gfmul(hash[i], base) ^ arr[i];
    }
    let is_lcp = |a: usize, c: usize, e: usize, len: usize| {
        let fst = hash[a] ^ hash[c] ^ hash[e];
        let snd = hash[a + len] ^ hash[c + len] ^ hash[e + len];
        gfmul(fst, pw[len]) == snd
    };
    let lcp = |a: usize, c: usize, e: usize| {
        let lim = (n - a).min(n - c).min(n - e);
        let mut pass = 0;
        let mut fail = lim + 1;
        while fail - pass > 1 {
            let mid  = (fail + pass) / 2;
            if is_lcp(a, c, e, mid) {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass
    };

    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [a, b, c, _d, e, f] = ints[..] else { panic!() };
        let a = a - 1;
        let c = c - 1;
        let e = e - 1;
        let maxlen = lcp(a, c, e);
        let maxlen = maxlen.min(b - a).min(f - e);
        if maxlen == (b - a).min(f - e) {
            println!("{}", yn(b - a < f - e));
            continue;
        }
        let fst = arr[a + maxlen] ^ arr[c + maxlen];
        let snd = arr[e + maxlen];
        println!("{}", yn(fst < snd));
    }
}
