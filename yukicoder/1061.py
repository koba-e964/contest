#!/usr/bin/env python3

import sys
readline = sys.stdin.buffer.readline
sys.setrecursionlimit(10 ** 7)

# The author read the editorial.


def join_b(a, b):
    ret = a.copy()
    last = ret.pop()
    ret.append(b.pop())
    ret.append(last)
    for elem in reversed(b):
        ret.append(elem)
    return ret


def make_a(n):
    assert n != 2
    if n == 0:
        return []
    if n == 5:
        a = [3, 4, 2, 2, 3, 3, 2, 4, 4]
        b = make_b(2)
        return join_b(a, b)
    if n % 2 == 1:
        a = []
        for i in range(n // 2, n):
            a.append(i)
        for i in range(n // 2, n):
            a.append(i)
            a.append(i)
        b = make_a(n // 2)
        return a + b
    assert n % 2 == 0
    a = []
    for i in range(n // 2, n):
        a.append(i)
    a.append(n - 1)
    for i in range(n // 2, n - 1):
        a.append(i)
        a.append(i)
    a.append(n - 1)
    b = make_b(n // 2)
    return join_b(a, b)


def make_b(n):
    assert n >= 2
    if n == 3:
        return [0, 0, 0, 2, 1, 2, 1, 1, 2]
    if n == 4:
        return [0, 0, 0, 3, 1, 3, 1, 1, 2, 2, 3, 2]
    if n % 2 == 1:
        a = []
        for i in range(n // 2, n):
            a.append(i)
        for i in range(n // 2, n):
            a.append(i)
            a.append(i)
        b = make_b(n // 2)
        return a + b
    assert n % 2 == 0
    a = []
    for i in range(n // 2, n):
        a.append(i)
    a.append(n - 1)
    for i in range(n // 2, n - 1):
        a.append(i)
        a.append(i)
    a.append(n - 1)
    b = make_a(n // 2)
    return b + a


n = int(readline())

if n == 2:
    print(-1)
    exit()

ans = make_a(n)
for i in range(3 * n):
    print(ans[i], end='')
    if i == 3 * n - 1:
        print()
    else:
        print(' ', end='')
