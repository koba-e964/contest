fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [d, mut f] = ints[..] else { panic!() };
    while f <= d {
        f += 7;
    }
    println!("{}", f - d);
}
