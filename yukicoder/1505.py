#!/usr/bin/env python3

n = int(input())
a = list(map(int, input().split()))

c = 0
s = 0
for b in a:
    if b == 1:
        c += 1
    else:
        c = 0
    s += c
print(n * (n + 1) // 2 - s)
