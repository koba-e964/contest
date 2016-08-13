a,u,q,v=$<.read.split.map &:to_f
p ((a-q).abs+(u-v).abs)/2
