use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

struct XorShift {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShift {
    fn new(x: u32) -> Self {
        XorShift {
            x: x,
            y: 1,
            z: 2,
            w: 3,
        }
    }
    fn generate(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w = self.w;
        self.w = (w ^ (w >> 19)) ^ (t ^ (t >> 8)); 
        self.w
    }
}

// Tags: low-memory
fn main() {
    let seed = get();
    let mut pass = 3u32 << 30;
    let mut fail = 1u32 << 30;
    let n = 10_000_001;
    while pass - fail > 1 {
        let mid = fail + (pass - fail) / 2;
        let mut rng = XorShift::new(seed);
        let mut le = 0;
        for _ in 0..n {
            let a = rng.generate();
            if a <= mid {
                le += 1;
            }
        }
        if le >= (n + 1) / 2 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
