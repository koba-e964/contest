#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

trait Int {
    fn ask(&self, a: usize, b: usize) -> (i64, i64) {
        println!("? {} {}", b + 1, a + 1);
        let a: i64 = get();
        let b: i64 = get();
        (a, b)
    }
    fn answer(&self, a: Vec<usize>) {
        print!("!");
        for a in a {
            print!(" {}", a);
        }
        println!();
    }
}

struct Wild;
impl Int for Wild {}

struct Local {
    n: usize,
    a: Vec<usize>,
}

impl Int for Local {
    fn ask(&self, a: usize, b: usize) -> (i64, i64) {
        let n = self.n;
        let x = self.a[a];
        let y = self.a[b];
        let mut v = vec![(y % n) as i64 - (x % n) as i64,
                         (y / n) as i64 - (x / n) as i64];
        v.sort();
        (v[0], v[1])
    }
    fn answer(&self, a: Vec<usize>) {
        assert_eq!(self.a, a);
    }
}

fn main() {
    let n: usize = get();
    let int = Wild;
    /*
    let mut a = vec![0; n * n - n];
    for i in 0..n * n - n {
        a[i] = n + i;
    }
    for i in 0..100 {
        let x = (5 * i + 3) % (n * n - n);
        let y = (7 * i + 1) % (n * n - n);
        a.swap(x, y);
    }
    let int = Local {
        n,
        a,
    };*/
    let mut res = vec![(0, 0); n * n - n];
    let mut hm = HashMap::new();
    hm.insert((0, 0), vec![0]);
    let mut freq = vec![0; 2 * n];
    freq[n] += 2;
    for i in 1..n * n - n {
        res[i] = int.ask(0, i);
        hm.entry(res[i]).or_insert(vec![]).push(i);
        let (x, y) = res[i];
        freq[(x + n as i64) as usize] += 1;
        freq[(y + n as i64) as usize] += 1;
    }
    let mut c = 0;
    let mut d = 0;
    for i in 0..2 * n {
        if freq[i] >= n {
            c = n + 1 - i;
            break;
        }
    }
    for i in 0..2 * n {
        if freq[i] % n > 0 {
            d = n - i;
            break;
        }
    }
    let mut truth = HashMap::new();
    for i in n..n * n {
        let dx = (i / n) as i64 - c as i64;
        let dy = (i % n) as i64 - d as i64;
        let (dx, dy) = if dx > dy {
            (dy, dx)
        } else {
            (dx, dy)
        };
        truth.entry((dx, dy)).or_insert(vec![]).push(i);
    }
    let mut ans = vec![0; n * n - n];
    let mut z = 0;
    let mut x = -(c as i64) + 1;
    let mut y = -(d as i64);
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    for i in 0..n * n - n {
        if res[i] == (x, y) {
            z = i;
            break;
        }
    }
    let indices = hm[&(-(c as i64) + 1, (n - d - 1) as i64)].clone();
    assert!(indices.len() <= 2);
    let mut bl = indices[0];
    if indices.len() == 2 {
        if int.ask(z, indices[0]) != (0, n as i64 - 1) {
            bl = indices[1];
        }
    }
    for (k, v) in hm {
        assert_eq!(v.len(), truth[&k].len());
        if v.len() >= 2 {
            let (x, y) = int.ask(bl, v[0]);
            let val = (x + n as i64 - 1) as usize + (y + 1) as usize * n;
            if truth[&k][0] == val {
                for i in 0..2 {
                    ans[v[i]] = truth[&k][i];
                }
            } else {
                for i in 0..2 {
                    ans[v[i]] = truth[&k][1 - i];
                }
            }
        } else {
            ans[v[0]] = truth[&k][0];
        }
    }
    int.answer(ans);
}
