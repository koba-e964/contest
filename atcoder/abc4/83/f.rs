fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    const W: usize = 500_000;
    let mut asum = 0i64;
    let mut hm = std::collections::HashMap::new();
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let [a, b] = ints[..] else { panic!() };
        asum += a as i64;
        *hm.entry((a, b)).or_insert(0) += 1;
    }
    let mut imos = vec![0; W + 1];
    for ((a, b), mul) in hm {
        for i in 0..=(W - (b - a)) / b {
            imos[i * b + (b - a)] -= b as i64 * mul;
        }
    }
    for i in 1..=W {
        imos[i] += imos[i - 1];
    }
    let q = getline().trim().parse::<i32>().unwrap();
    for _ in 0..q {
        let x = getline().trim().parse::<usize>().unwrap();
        println!("{}", n as i64 * x as i64 + asum + imos[x]);
    }
}
