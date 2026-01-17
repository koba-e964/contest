fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [p, q] = ints[..] else { panic!() };
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [x, y] = ints[..] else { panic!() };
    let x = x - p;
    let y = y - q;
    println!("{}", if (0..100).contains(&x) && (0..100).contains(&y) { "Yes" } else { "No" });
}
