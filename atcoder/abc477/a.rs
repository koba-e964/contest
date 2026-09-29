fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let c = getline().chars().next().unwrap();
    let t = "BYRB";
    let idx = t.chars().position(|x| x == c).unwrap();
    println!("{}", t.chars().nth(idx + 1).unwrap());
}
