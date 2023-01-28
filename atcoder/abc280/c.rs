fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().to_owned();
    let t = getline().to_owned();
    println!("{}", s.bytes().zip(t.bytes()).take_while(|&(a, b)| a == b).count() + 1);
}
