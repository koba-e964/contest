#!/usr/bin/env python3

s = input()
while len(s) % 3 != 0:
    s = '0' + s
f = [0] * 8
z = True
for i in range(len(s) // 3):
    x = 0
    for j in range(3 * i, 3 * i + 3):
        x = 16 * x + '0123456789ABCDEF'.index(s[j])
    c = 512
    for _ in range(4):
        if x // c != 0 or not z:
            f[x // c] += 1
            if x // c != 0:
                z = False
        x %= c
        c //= 8

ma = 0
for i in range(8):
    ma = max(ma, f[i])

v = []
for i in range(8):
    if f[i] == ma:
        v.append(i)

print(' '.join(map(str, v)))
