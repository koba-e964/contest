fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, k] = ints[..] else { panic!() };
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut b = a.clone();
    b.sort();
    let mut mi = n;
    let mut ma = 0;
    for i in 0..n {
        if a[i] != b[i] {
            mi = mi.min(i);
            ma = ma.max(i);
        }
    }
    println!("{}", yn(ma <= mi + k - 1));
}
