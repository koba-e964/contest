fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let q = ints[1];
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut acc2 = vec![0; n + 1];
    for i in 0..n {
        acc2[i + 1] = acc2[i] + acc[i + 1];
    }
    let f = |x: i64| {
        let mut pass = 0;
        let mut fail = n as i64 + 1;
        while fail - pass > 1 {
            let mid = (fail + pass) / 2;
            if mid * (mid + 1) / 2 <= x {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let prefix = acc2[pass as usize];
        let rest = x - pass * (pass + 1) / 2;
        prefix + acc[rest as usize]
    };
    for _ in 0..q {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [l, r] = ints[..] else { panic!() };
        println!("{}", f(r) - f(l - 1));
    }
}
