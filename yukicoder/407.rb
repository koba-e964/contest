require"prime";n,l=gets.split.map &:to_i;s=0;Prime.each(l/~-n){|v|s-=~l+v*~-n};p s
