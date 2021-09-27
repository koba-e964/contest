#!/usr/bin/env python3

n, m = map(int, input().split())
q = n // m
mod = pow(10, 9) + 7

moe = [1] * (q + 1)
fac = [1] * (q + 1)
for i in range(2, q + 1):
    if fac[i] != 1:
        continue
    fac[i] = i
    moe[i] = -1
    for j in range(2 * i, q + 1, i):
        fac[j] = i
        moe[j] = -moe[j] if j // i % i != 0 else 0

ans = 0
for i in range(1, q + 1):
    ans += (q // i) * (q // i - 1) * moe[i]

ans %= mod
for i in range(1, n - 1):
    ans = ans * i % mod

print(ans)
