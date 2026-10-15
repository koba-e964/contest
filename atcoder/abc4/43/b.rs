fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let [n, k] = ints[..] else { panic!() };
    let mut acc = 0;
    let mut x = -1;
    while acc < k {
        x += 1;
        acc += n + x;
    }
    println!("{x}");
}
