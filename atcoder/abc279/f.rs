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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut map: Vec<_> = (0..n).collect();
    let mut inv = map.clone();
    let mut pl = inv.clone();
    let mut buc = vec![vec![]; n];
    for i in 0..n {
        buc[i].push(i);
    }
    for _ in 0..q {
        let ty: i32 = get();
        let x = get::<usize>() - 1;
        if ty == 1 {
            let y = get::<usize>() - 1;
            let sw = buc[inv[x]].len() < buc[inv[y]].len();
            if sw {
                map.swap(inv[x], inv[y]);
                inv.swap(x, y);
            }
            let tmp = std::mem::replace(&mut buc[inv[y]], vec![]);
            for &idx in &tmp {
                pl[idx] = inv[x];
            }
            buc[inv[x]].extend_from_slice(&tmp);
        } else if ty == 2 {
            buc[inv[x]].push(pl.len());
            pl.push(inv[x]);
        } else {
            println!("{}", map[pl[x]] + 1);
        }
    }
}
