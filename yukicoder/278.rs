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
  let mut n : i64 = parse(getline().trim());
  let mut sum = 0i64;
  if n % 2 == 0 {
    n /= 2;
  }
  let mut x = 1i64;
  while x * x <= n {
    if n % x != 0 {
      x += 1;
      continue;
    }
    sum += x;
    if x * x != n {
      sum += n / x;
    }
    x += 1;
  }
  println!("{}", sum);
}
