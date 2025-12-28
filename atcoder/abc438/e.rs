fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();
    const B: usize = 31;
    let mut dbl = vec![vec![(0, 0); n]; B];
    for i in 0..n {
        dbl[0][i] = (i as i64 + 1, a[i]);
    }
    for i in 0..B - 1 {
        for j in 0..n {
            let (a1, b1) = dbl[i][j];
            let (a2, b2) = dbl[i][b1];
            dbl[i + 1][j] = (a1 + a2, b2);
        }
    }
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let t = ints[0];
        let mut b = ints[1] - 1;
        let mut ans = 0;
        for i in 0..B {
            if (t & 1 << i) != 0 {
                let (na, nb) = dbl[i][b];
                ans += na;
                b = nb;
            }
        }
        println!("{ans}");
    }
}
