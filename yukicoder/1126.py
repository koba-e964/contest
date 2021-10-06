#!/usr/bin/env python3

n, m = map(int, input().split())

a = 1
b = 1
MOD = pow(10, 9) + 7
for i in range(1, n + 2):
    b *= i
    b %= MOD
    a *= m - n + i
    a %= MOD

print(a * pow(b, MOD - 2, MOD) % MOD)
