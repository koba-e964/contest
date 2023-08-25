#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

# From https://qiita.com/AkariLuminous/items/3e2c80baa6d5e6f3abe9
# \sum_{0<=i<n} floor((ai+b)/m)
def floor_sum(n, m, a, b):
    ans = 0
    while True:
        if a >= m or a < 0:
            ans += n * (n - 1) * (a // m) // 2
            a %= m
        if b >= m or b < 0:
            ans += n * (b // m)
            b %= m
        y_max = a * n + b
        if y_max < m: break
        n, b, m, a = y_max // m, y_max % m, a, m
    return ans

# a/b > c/d
def count(a, b, c, d, n):
    lim = b * d // (a * d - b * c)
    n = min(n, lim)
    return n - (floor_sum(n + 1, b, a, 0) - floor_sum(n + 1, d, c, 0))

def calc(n, d, m, s):
    if m * d == 1 << s:
        return n
    if m * d < 1 << s:
        return count(1, d, m, 1 << s, n)
    return count(m, 1 << s, 1, d, n)

# https://yukicoder.me/problems/no/2440 (3.5)
# floor(an/b) = floor(cn/d) となる n の個数を求める問題。a/b > c/d を仮定してよくて、
# その場合は L を aL/b = cL/d + 1 を満たす実数とした場合に 0 <= n < L の範囲を探せばよくなる。
# 0 <= an/b - cn/d < 1 のときには floor(an/b) != floor(cn/d) と floor(an/b) - floor(cn/d) = 1 は同値であるため、
# floor_sum で高速に計算できる。
# Tags: floor-sum
q = int(readline())
for _ in range(q):
    n, d, m, s = map(int, readline().split())
    print(calc(n, d, m, s))
