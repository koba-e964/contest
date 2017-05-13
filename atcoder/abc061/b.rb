n, m = gets.split.map(&:to_i)
edges = Array.new(n, 0)
for _ in 0 ... m
  a, b = gets.split.map(&:to_i)
  edges[a - 1] += 1
  edges[b - 1] += 1
end
puts edges
