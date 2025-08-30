fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// https://yukicoder.me/problems/no/2236 (3)
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut e = vec![];
    for _ in 0..m {
        let ints = getline().trim().split_whitespace()
            .map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let [u, v] = ints[..] else { panic!() };
        e.push(1u64 << u | 1u64 << v);
    }
    let c = getline().trim().split_whitespace()
        .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut targ = 0u64;
    for i in 0..n {
        if c[i] == 1 {
            targ |= 2u64 << i;
        }
    }
    let mut ans = m as i32 + 1;
    let mut hm = std::collections::HashMap::new();
    for bits in 0..1 << (m / 2) {
        let mut flip = 0u64;
        let mut c = 0;
        for i in 0..m / 2 {
            if bits & (1 << i) != 0 {
                flip ^= e[i];
                c += 1;
            }
        }
        let entry = hm.entry(flip).or_insert(c);
        *entry = c.min(*entry);
    }
    for bits in 0..1 << (m - m / 2) {
        let mut flip = targ;
        let mut c = 0;
        for i in 0..m - m / 2 {
            if bits & (1 << i) != 0 {
                flip ^= e[i + m / 2];
                c += 1;
            }
        }
        if let Some(&v) = hm.get(&flip) {
            ans = ans.min(c + v);
        }
    }
    println!("{}", if ans > m as i32 { -1 } else { ans });
}
