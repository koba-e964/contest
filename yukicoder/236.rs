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
  let nums = getline().trim().split(" ").map(|x|parse(x)).collect::<Vec<f64>>();
  let sum = nums[0] + nums[1];
  println!("{:.10}", (nums[2] / nums[0]).min(nums[3] / nums[1]) * sum);
}
