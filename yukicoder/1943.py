#!/usr/bin/env python3

n = int(input())
s = input()
ans = 0
for i in range(n):
    if s[i] in 'AGCT':
       ans = i + 1
print(ans)
