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
    let mut sum = vec![0; m];
    let mut cnt = vec![0; m];
    for _ in 0..n {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let [a, b] = ints[..] else { panic!() };
        let a = a as usize - 1;
        sum[a] += b;
        cnt[a] += 1;
    }
    for i in 0..m {
        println!("{}", sum[i] as f64 / cnt[i] as f64);
    }
}
