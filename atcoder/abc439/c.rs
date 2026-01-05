fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n = getline().trim().parse::<usize>().unwrap();
    const W: usize = 3200;
    let mut dp = vec![0; n + 1];
    for x in 1..W {
        for y in x + 1..W {
            let v = x * x + y * y;
            if v <= n {
                dp[v] += 1;
            }
        }
    }
    let mut once = vec![];
    for i in 1..=n {
        if dp[i] == 1 {
            once.push(i);
        }
    }
    println!("{}", once.len());
    for v in once {
        println!("{v}");
    }
}
