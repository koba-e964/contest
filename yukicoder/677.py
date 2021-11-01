#!/usr/bin/env python3

n = int(input())
a = []
for i in range(n + 1):
    for j in range(n + 1):
        a.append(2 ** i * 5 ** j)
a.sort()
for b in a:
    print(b)
