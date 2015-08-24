fn getline() -> String {
  let mut ret = String::new();
  std::io::stdin().read_line(&mut ret).ok();
  return ret;
}

fn parse<T : std::str::FromStr>(s : &str) -> T {
  match s.parse::<T>() {
    Ok(t) => t,
    _    => panic!(),
  }
}

fn main() {
  let n = parse::<i32>(getline().trim());
  let mut acc = 12.0;
  for i in 0 .. 5 {
    acc /= (99 - i) as f64;
    acc *= (n - i) as f64;
  }
  println!("{:.10}", acc);
}
