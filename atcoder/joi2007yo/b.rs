fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut rem = (1i64 << 31) - 2;
    for _ in 0..28 {
        let x: usize = getline().trim().parse().unwrap();
        rem ^= 1 << x;
    }
    for i in 1..31 {
        if (rem & 1 << i) != 0 {
            println!("{}", i);
        }
    }
}
