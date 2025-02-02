fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut ans = "".to_string();
    for c in getline().trim().chars() {
        let x = match c {
            'N' => 'S',
            'S' => 'N',
            'W' => 'E',
            'E' => 'W',
            _ => panic!(),
        };
        ans.push(x);
    }
    println!("{ans}");
}
