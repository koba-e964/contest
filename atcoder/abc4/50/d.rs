fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let k = ints[1];
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap() % k)
        .collect::<Vec<_>>();
    a.sort(); a.dedup();
    let n = a.len();

    let mut mi = a[n - 1] - a[0];
    for i in 1..n {
        mi = mi.min(a[i - 1] - a[i] + k);
    }
    println!("{mi}");
}
