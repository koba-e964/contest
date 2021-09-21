#!/usr/bin/env python3

n = int(input())
a = list(map(int, input().split()))
ma = 0
for i in range(n):
    for j in range(i):
        ma = max(ma, a[i] ^ a[j])
print(ma)
