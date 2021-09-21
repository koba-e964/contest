#!/usr/bin/env python3

import sys

n = int(input())
a = list(map(int, input().split()))
c = 1
tot = 0
mo = 10 ** 9 + 7
for i in range(n):
    tot += c * a[i] % mo
    c = c * (n - i - 1) * pow(i + 1, mo - 2, mo) % mo
print(tot % mo)
