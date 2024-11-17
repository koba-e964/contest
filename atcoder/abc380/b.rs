fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    let a = s.split("|").map(|x| x.len()).collect::<Vec<_>>();
    for &a in &a[1..a.len() - 1] {
        print!("{} ", a);
    }
    println!();
}
