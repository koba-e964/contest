use std::collections::HashMap;

fn getline() -> String {
  let mut ret = String::new();
  std::io::stdin().read_line(&mut ret).ok();
  return ret;
}

fn parse(s : &str) -> i32 {
  let mut suit = HashMap::new();
  suit.insert('D', 0);
  suit.insert('C', 13);
  suit.insert('H', 26);
  suit.insert('S', 39);
  let mut mark = HashMap::new();
  mark.insert('A', 1);
  mark.insert('T', 10);
  mark.insert('J', 11);
  mark.insert('Q', 12);
  mark.insert('K', 13);
  for i in 2 .. 10 {
    mark.insert((48 + i) as u8 as char, i);
  }
  return suit.get(&s.chars().nth(0).unwrap()).unwrap() + mark.get(&s.chars().nth(1).unwrap()).unwrap();
}


fn main() {
  getline(); // discards n
  let mut cards : Vec<_> = getline().trim().split(" ").map(|x| x.to_string()).collect();
  cards.sort_by(|x, y| parse(x).cmp(&parse(y)));
  for i in 0 .. cards.len() {
    print!("{}{}", cards[i], if i == cards.len() - 1 {"\n"} else {" "});
  }
}
