#!/usr/bin/env python3

mo = 998244353
t = int(input())
for _ in range(t):
    n = int(input())
    print(pow(3, n - 1, mo) * 2 % mo)
