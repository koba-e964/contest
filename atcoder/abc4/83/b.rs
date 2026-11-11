fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, m] = ints[..] else { panic!() };
    let mut x = vec![0; n];
    for i in 0..n {
        x[i] = i as i32 + 1;
    }
    let k = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    
    for k in k {
        x[..k].reverse();
        for v in &mut x[..k] {
            *v *= -1;
        }
    }
    for v in x {
        println!("{v}");
    }
}
