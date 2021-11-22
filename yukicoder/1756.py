#!/usr/bin/env python3

def gcd(x, y):
    while y != 0:
        r = x % y
        x = y
        y = r
    return x


a, b, n = map(int, input().split())

if a == 0 or a == b:
    print(0)
    exit()

g = gcd(2 * a * b, b * b - a * a)
c = 2 * a * b // g
d = (b * b - a * a) // g


tot = 0

x = max(a * d, b * c)
y = a * c + b * d

if x < n and y < n:
    tot += (n - x) * (n - y) * 8

print(tot % 998244353)
