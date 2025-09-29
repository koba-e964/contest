fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(b: &[usize], k: i64) -> i64 {
    let mut f = vec![];
    for &x in b {
        while f.len() <= x {
            f.push(0);
        }
        f[x] += 1;
    }
    let mut rem = k;
    for i in (0..f.len()).rev() {
        let r = rem.min(f[i]);
        rem -= r;
        f[i] -= r;
        if i > 0 {
            f[i - 1] += r * 2;
        }
    }
    f.iter().sum()
}

fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [_, k, x] = ints[..] else { panic!() };
        let a = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let n = a.len();
        let mut pass = 0.0;
        let mut fail = 1.0e9;
        for _ in 0..61 {
            let mid = (pass + fail) / 2.0;
            let mut b = vec![];
            for i in 0..n {
                let u = a[i] as f64;
                if u < mid {
                    continue;
                }
                let v = (u / mid).log2().floor();
                b.push(v as usize);
            }
            let cnt = calc(&b, k);
            if cnt >= x {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        println!("{pass}");
    }
}
