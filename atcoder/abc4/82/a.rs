fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let c = getline().chars().filter(|&x| x == 'o').count();
    println!("{}", if c % 2 == 0 {
        "even"
    } else {
        "odd"
    });
}
