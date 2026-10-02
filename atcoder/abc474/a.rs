fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let x = getline().trim().parse::<usize>().unwrap();
    println!("{}", [0, 2, 3, 1][x]);
}
