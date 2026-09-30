fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [h, w] = ints[..] else { panic!() };
    println!("{}", if 10000 * w >= 22 * h * h {
        "Yes"
    } else {
        "No"
    });
}
