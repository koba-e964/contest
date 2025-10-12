fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut g = vec![0usize; n];
    for _ in 0..m {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        let [u, v] = ints[..] else { panic!() };
        g[u] |= 1 << v;
        g[v] |= 1 << u;
    }
    let mut e = vec![0; 1 << n];
    for bits in 0..1 << n {
        let mut tmp = 0;
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                tmp += (g[i] & bits).count_ones();
            }
        }
        e[bits] = tmp;
    }
    let mut ans = 100;
    for bits in 1..(1 << n) - 1 {
        ans = ans.min(e[bits] + e[(1 << n) - 1 - bits]);
    }
    println!("{}", ans / 2);
}
