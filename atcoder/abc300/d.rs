fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn psieve(n: usize) -> Vec<bool> {
    if n <= 1 {
        return vec![false; n + 1];
    }
    let mut pr = vec![true; n + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..n + 1 {
        if !pr[i] { continue; }
        for j in 2..n / i + 1 {
            pr[i * j] = false;
        }
    }
    pr
}

fn main() {
    let n = getline().trim().parse::<i64>().unwrap();
    const W: usize = 1_000_000;
    let tbl = psieve(W);
    let mut acc = vec![0; W + 2];
    for i in 0..W + 1 {
        acc[i + 1] = acc[i] + tbl[i] as i64;
    }
    let mut semi = vec![(0, 0); W + 1];
    for p in 2..W {
        if !tbl[p] { continue; }
        for q in p + 1..W {
            if !tbl[q] { continue; }
            if p * q >= W { break; }
            semi[p * q] = (p, q);
        }
    }
    let mut ans = 0;
    for i in 6..W {
        if semi[i] == (0, 0) { continue; }
        let (a, c) = semi[i];
        let ii = i as i64;
        if ii * ii > n {
            break;
        }
        let lim = n / (ii * ii) + 1;
        let lim = c.min(lim as usize);
        if a + 1 <= lim {
            ans += acc[lim] - acc[a + 1];
        }
    }
    println!("{ans}");
}
