s="DiscoPresentsDiscoveryChannelProgrammingContest2016"
n=gets.to_i
for b in 0 ... (50 + n) / n
  puts s[b * n ... b * n + n]
end
