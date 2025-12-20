use std::io::{self, Read};

fn calc(mut t: u64, mut l: u32, mut rem: i32) -> (u64, u32) {
    loop {
        if (t >> l) != 0 {
            t = 1;
            l += 1;
            continue;
        }
        if (t.count_ones() % 3) != 0 {
            t += 2;
            continue;
        }
        if rem == 0 {
            return (t, l);
        }
        t += 2;
        rem -= 1;
    }
}

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: i32 = s.trim().parse().unwrap();
    let (res, len) = calc(1, 1, n - 1);
    let mut out = String::new();
    for i in 0..len {
        let bit = 1u64 << (len - i - 1);
        if (res & bit) != 0 {
            out.push('5');
        } else {
            out.push('3');
        }
    }
    println!("{out}");
}
