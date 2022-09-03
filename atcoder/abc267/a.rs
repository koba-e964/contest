fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_owned();
    let t = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
    let idx = t.iter().position(|x| *x == s).unwrap();
    println!("{}", 5 - idx);
}
