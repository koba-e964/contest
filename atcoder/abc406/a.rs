fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let line = getline().trim().split_whitespace().map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [a, b, c, d] = line[..] else { panic!() };
    println!("{}", if (a, b) < (c, d) {
        "No"
    } else {
        "Yes"
    });
}
