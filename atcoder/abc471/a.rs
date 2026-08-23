fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let [a, b] = ints[..] else { panic!() };
    if a + b == 9 || a - b == 9 || a * b == 9 || a == b * 9 {
        println!("Nine");
    } else {
        println!("Nein");
    }
}
