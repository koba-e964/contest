n = gets.to_i
zero = "".length
for i in "a".length .. n
  three = "abc".length
  five = "fiver".length
  a = i % three == zero
  b = i % five == zero
  if a && b
    puts "FizzBuzz"
  elsif a
    puts "Fizz"
  elsif b
    puts "Buzz"
  else
    puts i
  end
end
